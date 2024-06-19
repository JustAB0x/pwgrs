# Maintainer: Box <box@sysn.co.uk>

pkgname=pwgrs
pkgver=1.2.0
pkgrel=1
pkgdesc="pwgrs is a command line tool which allows to creat secure passwords/secrets from the command line, written in rust, duh."
arch=('x86_64')
url="https://github.com/JustAB0x/pwgrs/"
license=('MIT' 'custom')
depends=('gcc-libs')
makedepends=('rust')
source=("$pkgname-$pkgver.tar.gz::https://github.com/JustAB0x/$pkgname/archive/v$pkgver.tar.gz")
sha512sums=('fd7e95b552851d1b20b779b3fff9b8f8342f63cfea792d10e5accd0a8c3cf64b9174a4b80d8eedc549e24a33c09a43e507e1e4c7a6971c8e4afa1c829a7494ef')
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
