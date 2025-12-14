#!/bin/bash
# Auto-detect the running cocoa-way instance's runtime directory
export XDG_RUNTIME_DIR=$(find /var/folders /tmp -type d -name "cocoa-way" 2>/dev/null | head -n 1)
export WAYLAND_DISPLAY=wayland-1

if [ -z "$XDG_RUNTIME_DIR" ]; then
    echo "Error: Could not find cocoa-way runtime directory. Please ensure cocoa-way is running."
    exit 1
fi

echo "Found cocoa-way at: $XDG_RUNTIME_DIR"
echo "Running waypipe command..."

# Execute waypipe with the provided arguments (e.g., ssh user@host niri)
# waypipe will automatically use the exported WAYLAND_DISPLAY and XDG_RUNTIME_DIR to connect to our compositor
exec waypipe "$@"
