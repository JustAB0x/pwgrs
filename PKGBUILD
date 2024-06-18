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
sha512sums=('2563fccab087cf151a92c00e84a6fcf1c25af6ae78c7ea6220e4b80b0634c72c1320fa327c694842f57ad42b321421e0748f52c0053a3288028d0972ec3ce5b6')
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
