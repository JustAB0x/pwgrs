# Maintainer: Box <box@sysn.co.uk>

pkgname=pwgrs
pkgver=2.0.2
pkgrel=1
pkgdesc="pwgrs is a command line tool which creates secure passwords/secrets from the command line with sensible defaults and a built-in copy-to-clipboard feature, written in rust."
arch=('x86_64' 'aarch64')
url="https://github.com/JustAB0x/pwgrs/"
license=('MIT' 'custom')
depends=('gcc-libs')
makedepends=('rust')
source=("$pkgname-$pkgver.tar.gz::https://github.com/JustAB0x/$pkgname/archive/v$pkgver.tar.gz")
sha512sums=('f376174cc0f7776b03be7fcdeb7b09fc58e676e7d78503879a39f54e6cd31e5fd2bce497ded0e656571d9003fece4582d1353faa1951326674db980d90b91c3e')
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
