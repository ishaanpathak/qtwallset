# Maintainer: Your Name <youremail@example.com>
pkgname=qtwallset
pkgver=0.1.1
pkgrel=1
pkgdesc="A tool to set Qtile wallpaper and regenerate matugen colors"
arch=('x86_64')
url="https://github.com/ishaanpathak/qtwallset"
license=('MIT')
depends=('qtile' 'matugen')
makedepends=('rust' 'cargo' 'git')
source=("$pkgname::git+$url.git#tag=v$pkgver")
sha256sums=('SKIP')  # Use SKIP for Git sources unless you manage checksums manually

build() {
  cd "$srcdir/$pkgname"
  cargo build --release --locked
}

package() {
  cd "$srcdir/$pkgname"
  install -Dm755 "target/release/qtwallset" "$pkgdir/usr/bin/qtwallset"
}
