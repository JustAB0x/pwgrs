# Maintainer: Box <box@sysn.co.uk>

pkgname=pwgrs
pkgver=1.3.2
pkgrel=1
pkgdesc="pwgrs is a command line tool which creates secure passwords/secrets from the command line with sensible defaults and a built-in copy-to-clipboard feature, written in rust."
arch=('x86_64')
url="https://github.com/JustAB0x/pwgrs/"
license=('MIT' 'custom')
depends=('gcc-libs')
makedepends=('rust')
source=("$pkgname-$pkgver.tar.gz::https://github.com/JustAB0x/$pkgname/archive/v$pkgver.tar.gz")
sha512sums=('a5adfa3228b809592f55f415fca54c8feb4e512883f93792f65578b588f3a52a51076d19c8440654a791f02b2fb9cbbd4e2207845262a5f592dcda9cbad903e5')
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
