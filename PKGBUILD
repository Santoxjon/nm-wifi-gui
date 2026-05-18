# Maintainer: Jon <jon@example.com>
pkgname=nm-wifi-gui
pkgver=0.1.0
pkgrel=1
pkgdesc="Lightweight GTK4/libadwaita NetworkManager WiFi GUI for Linux"
arch=('x86_64' 'aarch64')
url="https://github.com/Santoxjon/nm-wifi-gui"
license=('MIT')
depends=('gtk4' 'libadwaita' 'networkmanager' 'glib2')
makedepends=('rust' 'cargo' 'pkgconf' 'gcc')
source=("git+https://github.com/Santoxjon/nm-wifi-gui.git#tag=v${pkgver}")
sha256sums=('SKIP')

build() {
  cd nm-wifi-gui
  RUSTFLAGS="-C target-cpu=native" cargo build --release --locked
}

package() {
  cd nm-wifi-gui
  install -Dm755 target/release/nm-wifi-gui "${pkgdir}/usr/bin/nm-wifi-gui"
  install -Dm644 LICENSE "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
  install -Dm644 nm-wifi-gui.desktop "${pkgdir}/usr/share/applications/nm-wifi-gui.desktop"
}
