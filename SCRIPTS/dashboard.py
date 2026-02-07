#!/usr/bin/env python3
"""
S3M2P Dev Dashboard — Local Network Development Server Manager
Zero external dependencies. Run with: ./SCRIPTS/dev dashboard
Serves on port 9000, accessible from any device on the LAN.
"""

import json
import os
import signal
import socket
import subprocess
import sys
import threading
import time
from http.server import HTTPServer, BaseHTTPRequestHandler
from pathlib import Path

# ---------------------------------------------------------------------------
# Configuration — mirrors SCRIPTS/config.sh
# ---------------------------------------------------------------------------

REPO_ROOT = Path(__file__).resolve().parent.parent
LOG_DIR = REPO_ROOT / "TESTS" / "logs"
DASHBOARD_PORT = 9000

PROJECTS = {
    # id: (name, port, directory_relative_to_repo, group)
    "welcome":   {"name": "Welcome",        "port": 8080, "dir": "WELCOME",              "group": "MAIN"},
    "helios":    {"name": "Helios",          "port": 8081, "dir": "HELIOS",               "group": "MAIN"},
    "blog":      {"name": "Blog",            "port": 8085, "dir": "BLOG",                 "group": "MAIN"},
    "arch":      {"name": "Arch",            "port": 8087, "dir": "ARCH",                 "group": "MAIN"},
    "mcad":      {"name": "MCAD",            "port": 8088, "dir": "MCAD",                 "group": "MAIN"},
    "atlas":     {"name": "Atlas",           "port": 8089, "dir": "ATLAS",                "group": "MAIN"},

    "learn":     {"name": "Learn Hub",       "port": 8086, "dir": "LEARN",                "group": "LEARN HUB"},

    "ai":        {"name": "AI",              "port": 8100, "dir": "LEARN/AI",             "group": "LEARN"},
    "ubuntu":    {"name": "Ubuntu",          "port": 8101, "dir": "LEARN/UBUNTU",         "group": "LEARN"},
    "opencv":    {"name": "OpenCV",          "port": 8102, "dir": "LEARN/OPENCV",         "group": "LEARN"},
    "arduino":   {"name": "Arduino",         "port": 8103, "dir": "LEARN/ARDUINO",        "group": "LEARN"},
    "esp32":     {"name": "ESP32",           "port": 8104, "dir": "LEARN/ESP32",          "group": "LEARN"},
    "swarm":     {"name": "Swarm",           "port": 8105, "dir": "LEARN/SWARM_ROBOTICS", "group": "LEARN"},
    "slam":      {"name": "SLAM",            "port": 8106, "dir": "LEARN/SLAM",           "group": "LEARN"},
    "git":       {"name": "Git",             "port": 8107, "dir": "LEARN/GIT",            "group": "LEARN"},
    "ds":        {"name": "Data Structures", "port": 8108, "dir": "LEARN/DATA_STRUCTURES","group": "LEARN"},
    "python":    {"name": "Python",          "port": 8110, "dir": "LEARN/PYTHON",         "group": "LEARN"},
    "sensors":   {"name": "Sensors",         "port": 8083, "dir": "LEARN/SENSORS",        "group": "LEARN"},

    "pll":       {"name": "PLL",             "port": 8090, "dir": "TOOLS/PLL",            "group": "TOOLS"},
    "autocrate": {"name": "Autocrate",       "port": 8084, "dir": "TOOLS/AUTOCRATE",      "group": "TOOLS"},
    "power":     {"name": "Power",           "port": 8091, "dir": "TOOLS/POWER_CIRCUITS", "group": "TOOLS"},
    "spice":     {"name": "SPICE",           "port": 8123, "dir": "TOOLS/SPICE",          "group": "TOOLS"},

    "chladni":   {"name": "Chladni",         "port": 8082, "dir": "SIMULATION/CHLADNI",   "group": "SIMULATION"},
    "handtrack": {"name": "Handtrack",       "port": 8121, "dir": "SIMULATION/HANDTRACK", "group": "SIMULATION"},
    "powerlaw":  {"name": "Power Law",       "port": 8122, "dir": "SIMULATION/POWERLAW",  "group": "SIMULATION"},
}

GROUP_ORDER = ["MAIN", "LEARN HUB", "LEARN", "TOOLS", "SIMULATION"]

# ---------------------------------------------------------------------------
# Project Manager
# ---------------------------------------------------------------------------

class ProjectManager:
    """Manages trunk subprocess lifecycle for all projects."""

    def __init__(self):
        self.processes = {}      # id -> Popen
        self.statuses = {}       # id -> "stopped"|"building"|"running"|"error"|"unmanaged"
        self.exit_codes = {}     # id -> int|None
        self.lock = threading.Lock()

        LOG_DIR.mkdir(parents=True, exist_ok=True)

        # Initialize statuses — detect pre-existing processes on ports
        for pid, info in PROJECTS.items():
            if self._port_open(info["port"]):
                self.statuses[pid] = "unmanaged"
            else:
                self.statuses[pid] = "stopped"
            self.exit_codes[pid] = None

    def _port_open(self, port):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.settimeout(0.3)
            return s.connect_ex(("127.0.0.1", port)) == 0

    def start(self, project_id):
        with self.lock:
            if project_id not in PROJECTS:
                return False
            # Don't start if already running/building
            if self.statuses.get(project_id) in ("running", "building", "unmanaged"):
                return False

            info = PROJECTS[project_id]
            project_dir = REPO_ROOT / info["dir"]
            log_file = LOG_DIR / f"{project_id}.log"

            if not (project_dir / "index.html").exists() and not (project_dir / "Trunk.toml").exists():
                self.statuses[project_id] = "error"
                return False

            env = os.environ.copy()
            env["NO_COLOR"] = "true"

            with open(log_file, "w") as lf:
                try:
                    proc = subprocess.Popen(
                        ["trunk", "serve", "index.html", "--port", str(info["port"]), "--address", "0.0.0.0"],
                        cwd=str(project_dir),
                        stdout=lf,
                        stderr=subprocess.STDOUT,
                        env=env,
                        preexec_fn=os.setsid,
                    )
                except FileNotFoundError:
                    self.statuses[project_id] = "error"
                    return False

            self.processes[project_id] = proc
            self.statuses[project_id] = "building"
            self.exit_codes[project_id] = None
            return True

    def stop(self, project_id):
        with self.lock:
            proc = self.processes.get(project_id)
            if proc is None:
                # If unmanaged, try to kill whatever is on the port
                if self.statuses.get(project_id) == "unmanaged":
                    port = PROJECTS[project_id]["port"]
                    self._kill_port(port)
                    self.statuses[project_id] = "stopped"
                    return True
                return False

            try:
                pgid = os.getpgid(proc.pid)
                os.killpg(pgid, signal.SIGTERM)
            except (ProcessLookupError, OSError):
                pass

            # Wait up to 3 seconds for graceful shutdown
            try:
                proc.wait(timeout=3)
            except subprocess.TimeoutExpired:
                try:
                    pgid = os.getpgid(proc.pid)
                    os.killpg(pgid, signal.SIGKILL)
                except (ProcessLookupError, OSError):
                    pass

            del self.processes[project_id]
            self.statuses[project_id] = "stopped"
            self.exit_codes[project_id] = proc.returncode
            return True

    def _kill_port(self, port):
        """Kill whatever process is listening on a port."""
        try:
            result = subprocess.run(
                ["lsof", "-t", "-i", f":{port}"],
                capture_output=True, text=True, timeout=5
            )
            for pid_str in result.stdout.strip().split("\n"):
                if pid_str.strip():
                    try:
                        os.kill(int(pid_str.strip()), signal.SIGTERM)
                    except (ProcessLookupError, OSError, ValueError):
                        pass
        except (subprocess.TimeoutExpired, FileNotFoundError):
            pass

    def start_all(self):
        """Start all stopped projects with staggered delay."""
        def _staggered():
            for pid in PROJECTS:
                if self.statuses.get(pid) == "stopped":
                    self.start(pid)
                    time.sleep(1)
        threading.Thread(target=_staggered, daemon=True).start()

    def stop_all(self):
        for pid in list(PROJECTS.keys()):
            if self.statuses.get(pid) in ("running", "building", "unmanaged"):
                self.stop(pid)

    def start_group(self, group):
        def _staggered():
            for pid, info in PROJECTS.items():
                if info["group"] == group and self.statuses.get(pid) == "stopped":
                    self.start(pid)
                    time.sleep(1)
        threading.Thread(target=_staggered, daemon=True).start()

    def stop_group(self, group):
        for pid, info in PROJECTS.items():
            if info["group"] == group and self.statuses.get(pid) in ("running", "building", "unmanaged"):
                self.stop(pid)

    def update_statuses(self):
        """Called periodically by monitor thread."""
        with self.lock:
            for pid, info in PROJECTS.items():
                proc = self.processes.get(pid)
                port_up = self._port_open(info["port"])

                if proc is not None:
                    rc = proc.poll()
                    if rc is not None:
                        # Process exited
                        self.exit_codes[pid] = rc
                        del self.processes[pid]
                        if rc == 0:
                            self.statuses[pid] = "stopped"
                        else:
                            self.statuses[pid] = "error"
                    elif port_up:
                        self.statuses[pid] = "running"
                    else:
                        self.statuses[pid] = "building"
                elif self.statuses[pid] in ("unmanaged",):
                    if not port_up:
                        self.statuses[pid] = "stopped"
                elif self.statuses[pid] == "stopped" and port_up:
                    # Something started externally
                    self.statuses[pid] = "unmanaged"

    def get_status_json(self):
        counts = {"running": 0, "building": 0, "stopped": 0, "error": 0, "unmanaged": 0}
        projects = {}
        for pid, info in PROJECTS.items():
            status = self.statuses.get(pid, "stopped")
            counts[status] = counts.get(status, 0) + 1
            projects[pid] = {
                "name": info["name"],
                "port": info["port"],
                "group": info["group"],
                "status": status,
                "exitCode": self.exit_codes.get(pid),
            }
        return {"counts": counts, "projects": projects, "groups": GROUP_ORDER}

    def get_log(self, project_id, lines=100):
        log_file = LOG_DIR / f"{project_id}.log"
        if not log_file.exists():
            return ""
        try:
            with open(log_file, "r", errors="replace") as f:
                all_lines = f.readlines()
                return "".join(all_lines[-lines:])
        except OSError:
            return ""

    def shutdown(self):
        """Stop all managed processes — called on dashboard exit."""
        print("\nShutting down all managed processes...")
        for pid in list(self.processes.keys()):
            self.stop(pid)
        print("All processes stopped.")


# ---------------------------------------------------------------------------
# Monitor Thread
# ---------------------------------------------------------------------------

class MonitorThread(threading.Thread):
    def __init__(self, manager):
        super().__init__(daemon=True)
        self.manager = manager

    def run(self):
        while True:
            try:
                self.manager.update_statuses()
            except Exception:
                pass
            time.sleep(2)


# ---------------------------------------------------------------------------
# HTTP Handler
# ---------------------------------------------------------------------------

class DashboardHandler(BaseHTTPRequestHandler):
    manager = None  # Set by main()

    def log_message(self, format, *args):
        # Suppress default HTTP logging
        pass

    def do_GET(self):
        if self.path == "/":
            self._serve_html()
        elif self.path == "/api/status":
            self._json_response(self.manager.get_status_json())
        elif self.path.startswith("/api/logs/"):
            project_id = self.path.split("/")[-1]
            log_text = self.manager.get_log(project_id)
            self._json_response({"log": log_text})
        else:
            self.send_error(404)

    def do_POST(self):
        if self.path.startswith("/api/start-all"):
            self.manager.start_all()
            self._json_response({"ok": True})
        elif self.path.startswith("/api/stop-all"):
            self.manager.stop_all()
            self._json_response({"ok": True})
        elif self.path.startswith("/api/start-group/"):
            group = self.path.split("/")[-1].replace("%20", " ")
            self.manager.start_group(group)
            self._json_response({"ok": True})
        elif self.path.startswith("/api/stop-group/"):
            group = self.path.split("/")[-1].replace("%20", " ")
            self.manager.stop_group(group)
            self._json_response({"ok": True})
        elif self.path.startswith("/api/start/"):
            project_id = self.path.split("/")[-1]
            ok = self.manager.start(project_id)
            self._json_response({"ok": ok})
        elif self.path.startswith("/api/stop/"):
            project_id = self.path.split("/")[-1]
            ok = self.manager.stop(project_id)
            self._json_response({"ok": ok})
        else:
            self.send_error(404)

    def _json_response(self, data):
        body = json.dumps(data).encode()
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Access-Control-Allow-Origin", "*")
        self.end_headers()
        self.wfile.write(body)

    def _serve_html(self):
        body = DASHBOARD_HTML.encode()
        self.send_response(200)
        self.send_header("Content-Type", "text/html; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)


# ---------------------------------------------------------------------------
# Dashboard HTML/CSS/JS (inline)
# ---------------------------------------------------------------------------

DASHBOARD_HTML = r"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>S3M2P Dev Dashboard</title>
<style>
  :root {
    --bg: #0f1117;
    --card: #1a1d27;
    --card-hover: #22263a;
    --border: #2a2e3e;
    --text: #e0e0e0;
    --text-dim: #888;
    --green: #22c55e;
    --amber: #f59e0b;
    --red: #ef4444;
    --blue: #3b82f6;
    --gray: #6b7280;
  }
  * { margin: 0; padding: 0; box-sizing: border-box; }
  body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    background: var(--bg);
    color: var(--text);
    min-height: 100vh;
    padding: 1rem;
  }

  /* Top bar */
  .topbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 0.75rem;
    margin-bottom: 1.5rem;
    padding: 0.75rem 1rem;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 0.5rem;
  }
  .topbar h1 {
    font-size: 1.1rem;
    font-weight: 600;
    white-space: nowrap;
  }
  .counters {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
  }
  .counter {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    font-size: 0.85rem;
  }
  .counter .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    display: inline-block;
  }
  .global-actions {
    display: flex;
    gap: 0.5rem;
  }

  /* Buttons */
  button {
    background: var(--border);
    color: var(--text);
    border: 1px solid var(--border);
    padding: 0.4rem 0.75rem;
    border-radius: 0.35rem;
    cursor: pointer;
    font-size: 0.8rem;
    min-height: 44px;
    min-width: 44px;
    transition: background 0.15s;
  }
  button:hover { background: var(--card-hover); }
  button.start { border-color: var(--green); color: var(--green); }
  button.start:hover { background: rgba(34,197,94,0.15); }
  button.stop { border-color: var(--red); color: var(--red); }
  button.stop:hover { background: rgba(239,68,68,0.15); }

  /* Groups */
  .group {
    margin-bottom: 1.5rem;
  }
  .group-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.5rem;
    padding: 0 0.25rem;
  }
  .group-header h2 {
    font-size: 0.85rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-dim);
  }
  .group-actions {
    display: flex;
    gap: 0.35rem;
  }
  .group-actions button {
    font-size: 0.7rem;
    min-height: 32px;
    padding: 0.25rem 0.5rem;
  }

  /* Cards grid */
  .cards {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    gap: 0.5rem;
  }

  /* Card */
  .card {
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    padding: 0.75rem;
    cursor: pointer;
    transition: background 0.15s, border-color 0.15s;
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }
  .card:hover { background: var(--card-hover); }
  .card.selected { border-color: var(--blue); }

  .card-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .card-name {
    font-weight: 600;
    font-size: 0.9rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .status-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }
  .status-dot.stopped  { background: var(--gray); }
  .status-dot.building { background: var(--amber); animation: pulse 1.5s ease-in-out infinite; }
  .status-dot.running  { background: var(--green); }
  .status-dot.error    { background: var(--red); }
  .status-dot.unmanaged { background: var(--blue); }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.4; }
  }

  .card-bottom {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .card-port {
    font-size: 0.75rem;
    color: var(--text-dim);
  }
  .card-actions {
    display: flex;
    gap: 0.25rem;
  }
  .card-actions button {
    font-size: 0.7rem;
    min-height: 32px;
    min-width: 32px;
    padding: 0.2rem 0.4rem;
  }
  .card-actions a {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 32px;
    min-width: 32px;
    border: 1px solid var(--border);
    border-radius: 0.35rem;
    color: var(--text-dim);
    text-decoration: none;
    font-size: 0.75rem;
    transition: background 0.15s;
  }
  .card-actions a:hover { background: var(--card-hover); color: var(--text); }

  /* Log viewer */
  .log-panel {
    margin-top: 1rem;
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    overflow: hidden;
  }
  .log-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 0.75rem;
    border-bottom: 1px solid var(--border);
    font-size: 0.85rem;
    font-weight: 600;
  }
  .log-header button {
    font-size: 0.7rem;
    min-height: 32px;
    padding: 0.2rem 0.5rem;
  }
  .log-content {
    height: 300px;
    overflow-y: auto;
    padding: 0.5rem 0.75rem;
    font-family: "SF Mono", "Fira Code", "Cascadia Code", monospace;
    font-size: 0.75rem;
    line-height: 1.5;
    white-space: pre-wrap;
    word-break: break-all;
    color: var(--text-dim);
  }
  .log-content .err { color: var(--red); }
</style>
</head>
<body>

<div class="topbar">
  <h1>S3M2P Dev Dashboard</h1>
  <div class="counters" id="counters"></div>
  <div class="global-actions">
    <button class="start" onclick="api('start-all')">Start All</button>
    <button class="stop" onclick="api('stop-all')">Stop All</button>
  </div>
</div>

<div id="groups"></div>

<div class="log-panel">
  <div class="log-header">
    <span id="log-title">Select a project to view logs</span>
    <button onclick="refreshLog()">Refresh</button>
  </div>
  <div class="log-content" id="log-content">No project selected.</div>
</div>

<script>
const host = window.location.hostname;
let selectedProject = null;
let state = {};

function api(path, method) {
  method = method || (path.startsWith('start') || path.startsWith('stop') ? 'POST' : 'GET');
  return fetch('/api/' + path, { method: method }).then(r => r.json());
}

function selectProject(id) {
  selectedProject = id;
  document.querySelectorAll('.card').forEach(c => c.classList.remove('selected'));
  const el = document.getElementById('card-' + id);
  if (el) el.classList.add('selected');
  refreshLog();
}

function refreshLog() {
  if (!selectedProject) return;
  const info = state.projects && state.projects[selectedProject];
  const title = info ? info.name + ' :' + info.port : selectedProject;
  document.getElementById('log-title').textContent = 'Logs — ' + title;
  api('logs/' + selectedProject).then(d => {
    const el = document.getElementById('log-content');
    const lines = (d.log || '').split('\n');
    el.innerHTML = lines.map(line => {
      if (/error|panic|failed/i.test(line)) {
        return '<span class="err">' + esc(line) + '</span>';
      }
      return esc(line);
    }).join('\n');
    el.scrollTop = el.scrollHeight;
  });
}

function esc(s) {
  return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;');
}

function render(data) {
  state = data;
  // Counters
  const c = data.counts;
  document.getElementById('counters').innerHTML =
    counter('green', c.running, 'running') +
    counter('amber', c.building, 'building') +
    counter('gray', c.stopped, 'stopped') +
    counter('red', c.error, 'errors') +
    counter('blue', c.unmanaged || 0, 'unmanaged');

  // Groups
  const groupsEl = document.getElementById('groups');
  let html = '';
  for (const group of data.groups) {
    const ids = Object.keys(data.projects).filter(id => data.projects[id].group === group);
    if (ids.length === 0) continue;
    const encGroup = encodeURIComponent(group);
    html += '<div class="group">';
    html += '<div class="group-header"><h2>' + esc(group) + '</h2>';
    html += '<div class="group-actions">';
    html += '<button class="start" onclick="api(\'start-group/' + encGroup + '\',\'POST\')">Start</button>';
    html += '<button class="stop" onclick="api(\'stop-group/' + encGroup + '\',\'POST\')">Stop</button>';
    html += '</div></div>';
    html += '<div class="cards">';
    for (const id of ids) {
      const p = data.projects[id];
      const sel = id === selectedProject ? ' selected' : '';
      const url = 'http://' + host + ':' + p.port;
      html += '<div class="card' + sel + '" id="card-' + id + '" onclick="selectProject(\'' + id + '\')">';
      html += '<div class="card-top"><span class="card-name">' + esc(p.name) + '</span>';
      html += '<span class="status-dot ' + p.status + '" title="' + p.status + '"></span></div>';
      html += '<div class="card-bottom"><span class="card-port">:' + p.port + '</span>';
      html += '<div class="card-actions">';
      if (p.status === 'stopped' || p.status === 'error') {
        html += '<button class="start" onclick="event.stopPropagation();api(\'start/' + id + '\',\'POST\')">Start</button>';
      } else {
        html += '<button class="stop" onclick="event.stopPropagation();api(\'stop/' + id + '\',\'POST\')">Stop</button>';
      }
      if (p.status === 'running' || p.status === 'unmanaged') {
        html += '<a href="' + url + '" target="_blank" rel="noopener" onclick="event.stopPropagation()" title="Open">&#8599;</a>';
      }
      html += '</div></div></div>';
    }
    html += '</div></div>';
  }
  groupsEl.innerHTML = html;
}

function counter(color, count, label) {
  return '<div class="counter"><span class="dot" style="background:var(--' + color + ')"></span>' +
         count + ' ' + label + '</div>';
}

function poll() {
  api('status').then(render).catch(() => {});
  if (selectedProject) refreshLog();
}

poll();
setInterval(poll, 3000);
</script>
</body>
</html>
"""

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    manager = ProjectManager()
    DashboardHandler.manager = manager

    # Start monitor thread
    monitor = MonitorThread(manager)
    monitor.start()

    # Signal handlers for graceful shutdown
    def on_signal(signum, frame):
        manager.shutdown()
        sys.exit(0)

    signal.signal(signal.SIGINT, on_signal)
    signal.signal(signal.SIGTERM, on_signal)

    server = HTTPServer(("0.0.0.0", DASHBOARD_PORT), DashboardHandler)
    print(f"S3M2P Dev Dashboard running on http://0.0.0.0:{DASHBOARD_PORT}")
    print(f"Open from any device: http://<your-ip>:{DASHBOARD_PORT}")
    print("Press Ctrl+C to stop.")

    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        manager.shutdown()
        server.server_close()


if __name__ == "__main__":
    main()
