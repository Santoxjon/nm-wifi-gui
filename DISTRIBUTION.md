# Distribution Guide for Arch Linux

This guide will help you prepare nm-wifi-gui for distribution on Arch Linux and potentially the AUR.

## What's Been Set Up

✅ **PKGBUILD** - The recipe for building and installing on Arch Linux  
✅ **.desktop file** - Application launcher metadata  
✅ **LICENSE** - MIT license  
✅ **Enhanced Cargo.toml** - Metadata for crates.io and packagers  
✅ **Installation documentation** - User-friendly Arch installation guide  

## Next Steps

### 1. Push to GitHub (Required)

Update the PKGBUILD file with your actual repository URL:

```bash
# Edit PKGBUILD and update the url and source lines:
# url="https://github.com/Santoxjon/nm-wifi-gui"
# source=("git+https://github.com/Santoxjon/nm-wifi-gui.git#tag=v${pkgver}")
```

Then push to GitHub:
```bash
git init
git add -A
git commit -m "Initial commit: WiFi manager with Arch packaging"
git branch -M main
git remote add origin https://github.com/Santoxjon/nm-wifi-gui.git
git push -u origin main
```

Create a release tag:
```bash
git tag v0.1.0
git push origin v0.1.0
```

### 2. Test Local Installation (Recommended)

Test that the PKGBUILD works correctly:

```bash
cd /tmp
git clone https://github.com/Santoxjon/nm-wifi-gui.git
cd nm-wifi-gui
makepkg -si
```

Verify it runs:
```bash
nm-wifi-gui
```

### 3. Optional: Publish to Crates.io

Make your package available on the Rust package registry:

```bash
# Create an account at https://crates.io/
# Get an API token and configure it
cargo login

# Publish
cargo publish
```

### 4. Optional: Submit to AUR

Once tested, submit to the Arch User Repository for easy installation:

1. Create an account at https://aur.archlinux.org/
2. Upload SSH public key to your AUR account
3. Generate .SRCINFO file:
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```
4. Clone AUR package repo:
   ```bash
   git clone ssh+git://aur@aur.archlinux.org/nm-wifi-gui.git
   ```
5. Copy files to the cloned repository:
   ```bash
   cp PKGBUILD .SRCINFO nm-wifi-gui.desktop LICENSE /path/to/aur/repo/
   ```
6. Push to AUR:
   ```bash
   cd /path/to/aur/repo
   git add -A
   git commit -m "Initial import"
   git push
   ```

Users can then install with:
```bash
yay -S nm-wifi-gui
# or
paru -S nm-wifi-gui
```

## Installation Methods for Users

Users can now install your application via:

### From GitHub (using PKGBUILD)
```bash
git clone https://github.com/Santoxjon/nm-wifi-gui.git
cd nm-wifi-gui
makepkg -si
```

### From AUR (after submission)
```bash
yay -S nm-wifi-gui
paru -S nm-wifi-gui
```

### Direct from Cargo
```bash
cargo install --git https://github.com/Santoxjon/nm-wifi-gui.git
```

## Current Project Files

- **PKGBUILD** - Arch Linux build recipe
- **nm-wifi-gui.desktop** - Application menu entry
- **LICENSE** - MIT license text
- **INSTALL_ARCH.md** - User-facing Arch installation guide
- **Cargo.toml** - Enhanced with metadata (repository, keywords, categories)

## Troubleshooting the Build

If `makepkg` fails:

1. **Verify dependencies are installed:**
   ```bash
   sudo pacman -S --needed rustup gcc pkgconf gtk4 libadwaita networkmanager
   ```

2. **Update Rust:**
   ```bash
   rustup update
   ```

3. **Clear build cache:**
   ```bash
   cd /tmp/nm-wifi-gui
   cargo clean
   makepkg -Rsi
   ```

## Customization

You may want to customize:

- **Maintainer info in PKGBUILD** - Update email/username
- **Desktop categories** - Modify in `nm-wifi-gui.desktop`
- **Application name** - If you want to rename from `nm-wifi-gui`
- **Repository URL** - Throughout this guide, replace the example account with `Santoxjon`

## Support & Maintenance

Once published, consider:

- Keeping releases regularly tagged on GitHub
- Responding to AUR comments and bug reports
- Maintaining dependency version compatibility
- Publishing new versions consistently
