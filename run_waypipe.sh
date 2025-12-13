#!/bin/bash
# Wrapper to launch Waypipe with the correct local XDG_RUNTIME_DIR for macos-wayland-compositor

# Ensure TMPDIR ends with /
TMP="${TMPDIR%/}/"
export XDG_RUNTIME_DIR="${TMP}cocoa-way"
export WAYLAND_DISPLAY=wayland-1

# Look for waypipe in PATH
WAYPIPE_BIN="waypipe"
SOCKET_SOCK="/tmp/waypipe-bridge.sock"

# Verify waypipe exists
if ! command -v "$WAYPIPE_BIN" &> /dev/null; then
    echo "[Wrapper] Error: 'waypipe' not found in PATH."
    echo "[Wrapper] Please install 'waypipe-darwin' (rename binary to waypipe) and ensure it is in your PATH."
    exit 1
fi

echo "[Wrapper] Setting XDG_RUNTIME_DIR=$XDG_RUNTIME_DIR"
echo "[Wrapper] Running: $WAYPIPE_BIN --socket $SOCKET_SOCK $@"

exec "$WAYPIPE_BIN" --socket "$SOCKET_SOCK" "$@"
