#!/bin/bash
# Start the GitHub issue poller in the background
# Usage: ./start_poller.sh [start|stop|status|logs]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
POLLER_SCRIPT="$SCRIPT_DIR/poll_issues.py"
PID_FILE="$HOME/.claude/poller.pid"
LOG_FILE="$HOME/.claude/poller.log"

mkdir -p "$HOME/.claude"

start() {
    if [ -f "$PID_FILE" ]; then
        PID=$(cat "$PID_FILE")
        if kill -0 "$PID" 2>/dev/null; then
            echo "Poller already running (PID=$PID)"
            return 1
        fi
        rm -f "$PID_FILE"
    fi

    echo "Starting poller..."
    nohup python3 "$POLLER_SCRIPT" >> "$LOG_FILE" 2>&1 &
    echo $! > "$PID_FILE"
    echo "Poller started (PID=$!)"
    echo "Logs: $LOG_FILE"
}

stop() {
    if [ -f "$PID_FILE" ]; then
        PID=$(cat "$PID_FILE")
        if kill -0 "$PID" 2>/dev/null; then
            echo "Stopping poller (PID=$PID)..."
            kill "$PID"
            rm -f "$PID_FILE"
            echo "Stopped"
            return 0
        fi
        rm -f "$PID_FILE"
    fi
    echo "Poller not running"
}

status() {
    if [ -f "$PID_FILE" ]; then
        PID=$(cat "$PID_FILE")
        if kill -0 "$PID" 2>/dev/null; then
            echo "Poller running (PID=$PID)"
            return 0
        fi
        rm -f "$PID_FILE"
    fi
    echo "Poller not running"
    return 1
}

logs() {
    if [ -f "$LOG_FILE" ]; then
        tail -f "$LOG_FILE"
    else
        echo "No log file found"
    fi
}

case "${1:-start}" in
    start)  start ;;
    stop)   stop ;;
    status) status ;;
    logs)   logs ;;
    restart) stop; sleep 1; start ;;
    *)
        echo "Usage: $0 {start|stop|status|logs|restart}"
        exit 1
        ;;
esac
