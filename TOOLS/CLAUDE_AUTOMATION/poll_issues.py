#!/usr/bin/env python3
"""
Simple GitHub Issue Responder using Claude Max Plan.

This script polls GitHub for issues with the 'claude-auto' label and
responds to new user comments using Claude.

Design principles:
- Single-threaded polling (no race conditions)
- Idempotent via comment ID tracking (no duplicates ever)
- Uses your Claude Max subscription via CLI
- Minimal dependencies (just gh + claude CLI)
"""

import subprocess
import json
import time
import sqlite3
import sys
import os
from pathlib import Path
from datetime import datetime

# Configuration
POLL_INTERVAL = 10  # seconds
REPO = "Shivam-Bhardwaj/S3M2P"
LABEL = "claude-auto"
PROJECT_DIR = "/home/curious/S3M2P"
DB_PATH = Path.home() / ".claude" / "poll_issues.db"

# Signature patterns that indicate a bot/agent comment
BOT_SIGNATURES = [
    "Co-Authored-By: Claude",
    "Generated with [Claude Code]",
    "Generated with Claude Code",
    "🤖 Generated with",
    "🤖 **CLAUDE_TRIGGER**",  # Old GitHub workflow trigger
    # Legacy patterns from old agent comments
    "## Re-Planning",
    "## Re-planning",
    "## ✅ Execution Complete",
    "## ✅ Implementation",
    "### Root Cause Identified",
    "### Root Cause Analysis",
    "## 🎯 Execution Summary",
    "## 📋 Re-Planning Summary",
]


def log(msg: str, level: str = "INFO"):
    """Log with timestamp and level."""
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    print(f"[{timestamp}] [{level}] {msg}", flush=True)


def log_debug(msg: str):
    """Debug level log."""
    log(msg, "DEBUG")


def log_error(msg: str):
    """Error level log."""
    log(msg, "ERROR")


def init_db() -> sqlite3.Connection:
    """Initialize SQLite database for tracking processed comments."""
    DB_PATH.parent.mkdir(parents=True, exist_ok=True)
    conn = sqlite3.connect(str(DB_PATH))
    conn.execute("""
        CREATE TABLE IF NOT EXISTS processed_comments (
            comment_id INTEGER PRIMARY KEY,
            issue_number INTEGER NOT NULL,
            processed_at TEXT NOT NULL
        )
    """)
    conn.execute("""
        CREATE TABLE IF NOT EXISTS active_agents (
            issue_number INTEGER PRIMARY KEY,
            pid INTEGER NOT NULL,
            started_at TEXT NOT NULL
        )
    """)
    conn.commit()
    return conn


def is_processed(conn: sqlite3.Connection, comment_id: int) -> bool:
    """Check if comment has already been processed."""
    result = conn.execute(
        "SELECT 1 FROM processed_comments WHERE comment_id = ?",
        (comment_id,)
    ).fetchone()
    return result is not None


def mark_processed(conn: sqlite3.Connection, comment_id: int, issue_number: int):
    """Mark comment as processed."""
    conn.execute(
        "INSERT OR IGNORE INTO processed_comments (comment_id, issue_number, processed_at) VALUES (?, ?, ?)",
        (comment_id, issue_number, datetime.now().isoformat())
    )
    conn.commit()


def has_active_agent(conn: sqlite3.Connection, issue_number: int) -> bool:
    """Check if there's already an agent running for this issue."""
    result = conn.execute(
        "SELECT pid FROM active_agents WHERE issue_number = ?",
        (issue_number,)
    ).fetchone()

    if result:
        pid = result[0]
        # Check if process is still running
        try:
            os.kill(pid, 0)  # Signal 0 just checks if process exists
            return True
        except OSError:
            # Process is dead, clean up
            conn.execute("DELETE FROM active_agents WHERE issue_number = ?", (issue_number,))
            conn.commit()
    return False


def set_active_agent(conn: sqlite3.Connection, issue_number: int, pid: int):
    """Record that an agent is running for this issue."""
    conn.execute(
        "INSERT OR REPLACE INTO active_agents (issue_number, pid, started_at) VALUES (?, ?, ?)",
        (issue_number, pid, datetime.now().isoformat())
    )
    conn.commit()


def clear_active_agent(conn: sqlite3.Connection, issue_number: int):
    """Clear the active agent record for an issue."""
    conn.execute("DELETE FROM active_agents WHERE issue_number = ?", (issue_number,))
    conn.commit()


def get_labeled_issues() -> list[int]:
    """Get all open issues with the claude-auto label."""
    try:
        result = subprocess.run(
            ["gh", "api", f"repos/{REPO}/issues",
             "-q", f'.[] | select(.state == "open") | select(.labels[].name == "{LABEL}") | .number'],
            capture_output=True, text=True, timeout=30
        )
        if result.returncode != 0:
            log(f"Error fetching issues: {result.stderr}")
            return []

        numbers = [int(n) for n in result.stdout.strip().split('\n') if n.strip()]
        return numbers
    except Exception as e:
        log(f"Exception fetching issues: {e}")
        return []


def get_issue_body(issue_number: int) -> dict | None:
    """Get the issue body as a pseudo-comment for new issues."""
    try:
        result = subprocess.run(
            ["gh", "api", f"repos/{REPO}/issues/{issue_number}"],
            capture_output=True, text=True, timeout=30
        )
        if result.returncode != 0 or not result.stdout.strip():
            return None

        issue = json.loads(result.stdout)
        # Return issue body as a pseudo-comment with negative ID to distinguish
        return {
            "id": -issue_number,  # Negative to distinguish from real comments
            "body": issue.get("body", ""),
            "user": issue.get("user", {}),
            "created_at": issue.get("created_at", ""),
        }
    except Exception as e:
        log(f"Exception fetching issue #{issue_number}: {e}")
        return None


def get_last_comment(issue_number: int) -> dict | None:
    """Get the most recent comment on an issue, or issue body if no comments."""
    try:
        result = subprocess.run(
            ["gh", "api", f"repos/{REPO}/issues/{issue_number}/comments", "--jq", ".[-1]"],
            capture_output=True, text=True, timeout=30
        )
        if result.returncode != 0 or not result.stdout.strip():
            # No comments - check if this is a new issue we should respond to
            return get_issue_body(issue_number)

        return json.loads(result.stdout)
    except Exception as e:
        log(f"Exception fetching comments for #{issue_number}: {e}")
        return None


def is_bot_comment(comment: dict) -> bool:
    """Check if comment is from a bot or agent."""
    # Check user type
    user = comment.get("user", {})
    if user.get("type") == "Bot":
        return True
    if user.get("login", "").endswith("[bot]"):
        return True

    # Check for signature patterns in body
    body = comment.get("body", "")
    for sig in BOT_SIGNATURES:
        if sig in body:
            return True

    return False


def spawn_claude(issue_number: int) -> subprocess.Popen | None:
    """Spawn Claude to respond to an issue."""
    prompt = f"""You are responding to GitHub issue #{issue_number} in the {REPO} repository.

INSTRUCTIONS:
1. First, read the issue and all comments:
   gh issue view {issue_number} --repo {REPO} --json title,body,comments

2. Understand what the user is asking or discussing.

3. Provide a helpful response. You can:
   - Answer questions
   - Investigate code in the repository
   - Suggest fixes or improvements
   - Create PRs if appropriate

4. Post your response as a comment:
   gh issue comment {issue_number} --repo {REPO} --body "<your response>"

5. IMPORTANT: Always end your comment with this signature:

   ---
   🤖 Generated with [Claude Code](https://claude.com/claude-code)

   Co-Authored-By: Claude <noreply@anthropic.com>

Be helpful, concise, and actionable."""

    # Create log file for this session
    log_path = Path.home() / ".claude" / "logs" / f"issue-{issue_number}.log"
    log_path.parent.mkdir(parents=True, exist_ok=True)

    try:
        # Use PIPE for stdin and communicate the prompt
        # This ensures the full prompt is passed before the process reads it
        log_file = open(log_path, 'w')
        process = subprocess.Popen(
            ["claude", "-p", "-", "--permission-mode", "bypassPermissions"],
            cwd=PROJECT_DIR,
            stdin=subprocess.PIPE,
            stdout=log_file,
            stderr=subprocess.STDOUT,
            text=True
        )

        # Write prompt to stdin and close it (signals EOF)
        # Do this in a non-blocking way so we don't wait for process to finish
        def write_prompt():
            try:
                process.stdin.write(prompt)
                process.stdin.close()
            except Exception as e:
                log_error(f"#{issue_number}: Error writing prompt: {e}")

        import threading
        writer = threading.Thread(target=write_prompt, daemon=True)
        writer.start()

        log(f"#{issue_number}: Claude output logging to {log_path}")
        return process
    except Exception as e:
        log(f"Error spawning Claude for #{issue_number}: {e}")
        return None


def poll_once(conn: sqlite3.Connection):
    """Run one poll cycle."""
    log_debug("Starting poll cycle...")
    issues = get_labeled_issues()

    if not issues:
        log_debug("No issues with 'claude-auto' label found")
        return

    log_debug(f"Found {len(issues)} issues with 'claude-auto' label: {issues}")

    for issue_number in issues:
        # Skip if agent already running for this issue
        if has_active_agent(conn, issue_number):
            log_debug(f"#{issue_number}: Skipping - agent already running")
            continue

        # Get last comment
        log_debug(f"#{issue_number}: Fetching latest comment...")
        comment = get_last_comment(issue_number)
        if not comment:
            log_debug(f"#{issue_number}: No comment found")
            continue

        comment_id = comment.get("id")
        if not comment_id:
            log_debug(f"#{issue_number}: Comment has no ID")
            continue

        # Skip if already processed
        if is_processed(conn, comment_id):
            log_debug(f"#{issue_number}: Comment {comment_id} already processed")
            continue

        # Skip bot/agent comments
        if is_bot_comment(comment):
            log(f"#{issue_number}: Skipping bot/agent comment {comment_id}")
            mark_processed(conn, comment_id, issue_number)
            continue

        # This is a new user comment - respond!
        user = comment.get("user", {}).get("login", "unknown")
        body_preview = comment.get("body", "")[:100].replace("\n", " ")
        log(f"#{issue_number}: NEW COMMENT from @{user} (id={comment_id})")
        log(f"#{issue_number}: Preview: {body_preview}...")

        # Mark as processed BEFORE spawning (idempotent)
        mark_processed(conn, comment_id, issue_number)

        # Spawn Claude
        log(f"#{issue_number}: Spawning Claude agent...")
        process = spawn_claude(issue_number)
        if process:
            set_active_agent(conn, issue_number, process.pid)
            log(f"#{issue_number}: Claude agent started (PID={process.pid})")
            log(f"#{issue_number}: Logs: ~/.claude/logs/issue-{issue_number}.log")
        else:
            log_error(f"#{issue_number}: Failed to spawn Claude agent")

    log_debug("Poll cycle complete")


def cleanup_dead_agents(conn: sqlite3.Connection):
    """Clean up records for agents that have exited."""
    rows = conn.execute("SELECT issue_number, pid, started_at FROM active_agents").fetchall()
    if rows:
        log_debug(f"Checking {len(rows)} active agents...")
    for issue_number, pid, started_at in rows:
        try:
            os.kill(pid, 0)
            log_debug(f"#{issue_number}: Agent PID={pid} still running (started {started_at})")
        except OSError:
            log(f"#{issue_number}: Agent (PID={pid}) has exited - cleaning up")
            clear_active_agent(conn, issue_number)


def seed_existing_comments(conn: sqlite3.Connection):
    """Pre-seed database with all existing comments to avoid reprocessing."""
    log("Seeding existing comments...")
    issues = get_labeled_issues()
    seeded = 0

    for issue_number in issues:
        try:
            result = subprocess.run(
                ["gh", "api", f"repos/{REPO}/issues/{issue_number}/comments", "--jq", ".[].id"],
                capture_output=True, text=True, timeout=30
            )
            if result.returncode == 0:
                for line in result.stdout.strip().split('\n'):
                    if line.strip():
                        comment_id = int(line)
                        if not is_processed(conn, comment_id):
                            mark_processed(conn, comment_id, issue_number)
                            seeded += 1
        except Exception as e:
            log(f"Error seeding comments for #{issue_number}: {e}")

    if seeded > 0:
        log(f"Seeded {seeded} existing comments")


def main():
    """Main polling loop."""
    print("=" * 60)
    log("Starting GitHub Issue Poller")
    log(f"  Repo: {REPO}")
    log(f"  Label: {LABEL}")
    log(f"  Poll interval: {POLL_INTERVAL}s")
    log(f"  Database: {DB_PATH}")
    log(f"  Project dir: {PROJECT_DIR}")
    log(f"  Claude logs: ~/.claude/logs/issue-<N>.log")
    print("=" * 60)

    conn = init_db()

    # Seed existing comments on first run to avoid reprocessing
    seed_existing_comments(conn)

    log("Entering main polling loop...")
    poll_count = 0
    try:
        while True:
            poll_count += 1
            try:
                cleanup_dead_agents(conn)
                poll_once(conn)
            except Exception as e:
                log_error(f"Error in poll cycle: {e}")
                import traceback
                log_error(traceback.format_exc())

            # Periodic status update every 30 polls (5 minutes with 10s interval)
            if poll_count % 30 == 0:
                active = conn.execute("SELECT COUNT(*) FROM active_agents").fetchone()[0]
                processed = conn.execute("SELECT COUNT(*) FROM processed_comments").fetchone()[0]
                log(f"Status: {poll_count} polls, {active} active agents, {processed} processed comments")

            time.sleep(POLL_INTERVAL)
    except KeyboardInterrupt:
        log("Shutting down (Ctrl+C)...")
    finally:
        conn.close()
        log("Database connection closed")


if __name__ == "__main__":
    main()
