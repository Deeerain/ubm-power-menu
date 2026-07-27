# Maintainer: deerain <1deerain1@gmail.com>

pkgname=dummy-power-menu
pkgver=0.1.0
pkgrel=1
pkgdesc="Simple power menu for Hyprland"
arch=(x86_64)
url="https://github.com/deeerain/ubm-power-menu"
license=(MIT)
depends=(gtk4 gtk4-layer-shell ttf-nerd-fonts-symbols)
makedepends=(cargo rust pkgconf)
source=("$pkgname::git+https://github.com/deeerain/ubm-power-menu.git#branch=main")
sha512sums=(SKIP)

build() {
  cd "$srcdir/$pkgname"
  cargo build --release --locked
}

package() {
  cd "$srcdir/$pkgname"

  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
  install -Dm644 config.json "$pkgdir/usr/share/$pkgname/config.json"
}
