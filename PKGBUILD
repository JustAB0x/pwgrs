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
sha512sums=('f5c68215f31e1c93c2817d853dc1da81932f25730256759bf8190375f2a789ed498ecc3c22c6a0f10d728460633151d7bfe9ff281181cde0216e03245d6f8886')
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
