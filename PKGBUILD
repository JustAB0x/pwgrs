# Maintainer: Box <box@sysn.co.uk>

pkgname=pwgrs
pkgver=1.1.0
pkgrel=1
pkgdesc="pwgrs is a command line tool which allows to creat secure passwords/secrets from the command line, written in rust, duh."
arch=('x86_64')
url="https://github.com/JustAB0x/pwgrs/"
license=('MIT')
depends=('gcc-libs')
makedepends=('rust')
source=("$pkgname-$pkgver.tar.gz::https://github.com/JustAB0x/$pkgname/archive/v$pkgver.tar.gz")
sha512sums=('3559703a046bb5a8c80da93e79f94663359b1b24ccb2f1c890acc5872899e9e594274674903567c321022e7058f810a2642b03fa5e46860c6683b59998f9ad01')
options=(!lto)

prepare() {
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
