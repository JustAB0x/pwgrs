# Maintainer: Box <box@sysn.co.uk>

pkgname=pwgen-rs
pkgver=1.1.0
pkgrel=1
pkgdesc="$pkgname is a command line tool which allows to creat secure passwords/secrets from the command line."
arch=('x86_64')
url="https://github.com/JustAB0x/pwgen-rs/"
license=('MIT')
depends=('gcc-libs')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/JustAB0x/$pkgname/archive/v$pkgver.tar.gz")
sha512sums=('1798f5432b044711fb64fdcf640aab6559c5f38d9959f8ef41649855c652bc0a77b59ca321c9420b07228dabc904ee6d2e6637576089f3dee606e0e8694a238c')
options=(!lto)

build() {
  cd "$pkgname-$pkgver"

  cargo build --release --locked
}


package() {
  cd "$pkgname-$pkgver"

  install -Dm755 "target/release/pwgen-rs" "$pkgdir/usr/bin/pwgen-rs"

  install -Dm644 "README.md" "$pkgdir/usr/share/doc/${pkgname}/README.md"
  install -Dm644 "LICENSE-MIT" "$pkgdir/usr/share/licenses/${pkgname}/LICENSE-MIT"
}
