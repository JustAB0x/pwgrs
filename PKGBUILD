# Maintainer: Box <box@sysn.co.uk>

pkgname=pwgrs
pkgver=1.2.0
pkgrel=1
pkgdesc="pwgrs is a command line tool which allows to creat secure passwords/secrets from the command line with smart defaults and a built-in copy-to-clipboard feature, written in rust, duh."
arch=('x86_64')
url="https://github.com/JustAB0x/pwgrs/"
license=('MIT' 'custom')
depends=('gcc-libs')
makedepends=('rust')
source=("$pkgname-$pkgver.tar.gz::https://github.com/JustAB0x/$pkgname/archive/v$pkgver.tar.gz")
sha512sums=('5b5de3f0ee0ae7763282c1932c264c3ea351f2966f265ea2d8506a20d4b246e635ea26eb32d87ac4b5e183b325e1cbad086d278aae8f8e62c5c8fd8693914082')
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
