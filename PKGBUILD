# Maintainer: Box <box@sysn.co.uk>

pkgname=pwgrs
pkgver=2.0.0
pkgrel=1
pkgdesc="pwgrs is a command line tool which creates secure passwords/secrets from the command line with sensible defaults and a built-in copy-to-clipboard feature, written in rust."
arch=('x86_64' 'aarch64')
url="https://github.com/JustAB0x/pwgrs/"
license=('MIT' 'custom')
depends=('gcc-libs')
makedepends=('rust')
source=("$pkgname-$pkgver.tar.gz::https://github.com/JustAB0x/$pkgname/archive/v$pkgver.tar.gz")
sha512sums=('4e3030fab75ccce3965954cbe1f68fcd2c3e98dba9aea12b67d1f5d50f8f2be131df7f90aec61c48c5f6c591d57c163dccea12389cf41cd93aedddb6a9229e56')
options=(!lto)

prepare() {
  cd "$pkgname-$pkgver"
  cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
  cd "$pkgname-$pkgver"

  cargo build --frozen --release --all-features
}


package() {
  cd "$pkgname-$pkgver"

  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

  install -Dm644 "README.md" "$pkgdir/usr/share/doc/${pkgname}/README.md"
  install -Dm644 "LICENSE-MIT" "$pkgdir/usr/share/licenses/${pkgname}/LICENSE-MIT"
  install -Dm644 "UNLICENSE" "$pkgdir/usr/share/licenses/${pkgname}/UNLICENSE"
}
