# Arch Linux Installation

This guide explains how to install nm-wifi-gui on Arch Linux and Arch-based distributions.

## Prerequisites

Install required build and runtime dependencies:

```bash
sudo pacman -S --needed rustup gcc pkgconf gtk4 libadwaita networkmanager
rustup default stable
```

## Installation Methods

### Option 1: Using PKGBUILD (Local Build)

Clone the repository and build locally:

```bash
git clone https://github.com/Santoxjon/nm-wifi-gui.git
cd nm-wifi-gui
makepkg -si
```

The PKGBUILD currently tracks the `main` branch so it works before the first release tag exists.

The `-s` flag builds with dependencies, and `-i` installs the package.

### Option 2: Direct Installation from Cargo

```bash
cargo install --path .
```

This installs the binary to `~/.cargo/bin/nm-wifi-gui`. Ensure `~/.cargo/bin` is in your `$PATH`.

### Option 3: Manual Installation

Build and install manually:

```bash
cargo build --release
sudo install -Dm755 target/release/nm-wifi-gui /usr/local/bin/nm-wifi-gui
sudo install -Dm644 nm-wifi-gui.desktop /usr/local/share/applications/nm-wifi-gui.desktop
sudo install -Dm644 LICENSE /usr/local/share/licenses/nm-wifi-gui/LICENSE
```

## Post-Installation

### Running the Application

From terminal:
```bash
nm-wifi-gui
```

Or find it in your application menu.

### Hyprland Configuration

For Hyprland users, add these window rules to `~/.config/hypr/hyprland.conf`:

```ini
windowrule = float, class:^(com\.jon\.nmwifigui)$
windowrule = center, class:^(com\.jon\.nmwifigui)$
windowrule = size 420 560, class:^(com\.jon\.nmwifigui)$
windowrule = pin, class:^(com\.jon\.nmwifigui)$
```

Or use the helper script:
```bash
./scripts/run-hypr-floating.sh
```

## Uninstallation

If installed via `makepkg`:
```bash
sudo pacman -R nm-wifi-gui
```

If installed via `cargo install`:
```bash
cargo uninstall nm-wifi-gui
```

If installed manually:
```bash
sudo rm /usr/local/bin/nm-wifi-gui
sudo rm /usr/local/share/applications/nm-wifi-gui.desktop
sudo rm -rf /usr/local/share/licenses/nm-wifi-gui
```

## AUR Submission (Optional)

To publish to the Arch User Repository (AUR), follow these steps:

1. Create an AUR account at https://aur.archlinux.org/
2. Generate an SSH key and upload it to AUR
3. Clone the AUR package repository:
   ```bash
   git clone ssh+git://aur@aur.archlinux.org/nm-wifi-gui.git
   ```
4. Add the PKGBUILD, nm-wifi-gui.desktop, and .SRCINFO files
5. Run `mksrcinfo` to generate .SRCINFO:
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```
6. Commit and push:
   ```bash
   git add -A
   git commit -m "Initial commit"
   git push
   ```

## Troubleshooting

### Permission Denied When Connecting to Networks

The application uses `nmcli` which requires NetworkManager to be running:
```bash
sudo systemctl start NetworkManager
sudo systemctl enable NetworkManager  # For auto-start on boot
```

### Missing Dependencies at Runtime

If you see library errors, install missing GTK4/libadwaita dependencies:
```bash
sudo pacman -S gtk4 libadwaita
```

### Build Failures

Ensure Rust is up to date:
```bash
rustup update
```

Clean and rebuild:
```bash
cargo clean
cargo build --release
```
