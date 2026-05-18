#!/usr/bin/env sh
set -eu

# Launch as a floating centered window in Hyprland.
# pin keeps the window above tiled layout and visible across workspaces.
PROJECT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
BIN_PATH=""

if command -v nm-wifi-gui >/dev/null 2>&1; then
	BIN_PATH="nm-wifi-gui"
elif [ -x "$PROJECT_DIR/target/release/nm-wifi-gui" ]; then
	BIN_PATH="$PROJECT_DIR/target/release/nm-wifi-gui"
elif [ -x "$PROJECT_DIR/target/debug/nm-wifi-gui" ]; then
	BIN_PATH="$PROJECT_DIR/target/debug/nm-wifi-gui"
fi

if [ -n "$BIN_PATH" ]; then
	hyprctl dispatch exec "[float; center; size 420 560; pin] $BIN_PATH"
else
	hyprctl dispatch exec "[float; center; size 420 560; pin] sh -lc 'cd \"$PROJECT_DIR\" && cargo run'"
fi
