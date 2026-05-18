# nm-wifi-gui

A lightweight NetworkManager WiFi GUI for Linux (Wayland-friendly), built with Rust + GTK4/libadwaita and using NetworkManager (`nmcli`) as backend.

## Features (v0.1)

- Scan available WiFi networks
- Sort networks by:
  - connected first
  - signal strength (descending)
- Show per network:
  - SSID
  - signal percent
  - security type
- Click network to connect:
  - known/saved network: connect directly
  - secure unsaved network: password dialog
- Auto-refresh scan every 5 seconds
- Manual refresh button

## Architecture

- `src/backend/mod.rs`: backend trait (`WifiBackend`) for future D-Bus migration
- `src/backend/nmcli.rs`: current `nmcli` implementation
- `src/models.rs`: network model
- `src/ui/window.rs`: GTK4/libadwaita UI and interactions
- `src/main.rs`: app entrypoint

## Requirements

- Linux with NetworkManager
- `nmcli` available in PATH
- GTK4 + libadwaita development libraries
- Rust toolchain (stable)

Arch/CachyOS packages (example):

```bash
sudo pacman -S --needed rustup gcc pkgconf gtk4 libadwaita networkmanager
rustup default stable
```

## Build

```bash
cargo build --release
```

## Installation

### Arch Linux / Arch-based Systems

See [INSTALL_ARCH.md](INSTALL_ARCH.md) for detailed installation instructions on Arch Linux, including:
- Using PKGBUILD for system-wide installation
- Installing via cargo
- Manual installation
- AUR submission steps

Quick install via PKGBUILD:
```bash
makepkg -si
```

### Other Linux Distributions

```bash
cargo install --path .
```

## Hyprland behavior (floating + centered + above tiled)

Wayland clients cannot reliably force global compositor placement/z-order.
For Hyprland, configure this via window rules.

### One-shot launch (from terminal)

```bash
hyprctl dispatch exec "[float; center; size 420 560; pin] nm-wifi-gui"
```

Or use the helper script:

```bash
./scripts/run-hypr-floating.sh
```

### Permanent Hyprland rules

Add to `~/.config/hypr/hyprland.conf`:

```ini
windowrule = float, class:^(com\.jon\.nmwifigui)$
windowrule = center, class:^(com\.jon\.nmwifigui)$
windowrule = size 420 560, class:^(com\.jon\.nmwifigui)$
windowrule = pin, class:^(com\.jon\.nmwifigui)$
```

Reload Hyprland config:

```bash
hyprctl reload
```

## Notes

- Passwords are passed to `nmcli` for connection setup and then managed by NetworkManager.
- This first version uses `nmcli` intentionally for simplicity; backend is trait-based so you can migrate to `zbus` later with minimal UI changes.
