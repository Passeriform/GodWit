# Maintainer: Utkarsh Bhardwaj (Passeriform) <passeriform.ub@gmail.com>
pkgname=godwit
pkgver=0.1.1
pkgrel=1
pkgdesc="A hackable yet sane project manager and automation suite."
arch=('x86_64' 'i686' 'armv6h' 'armv7h')
url="https://www.passeriform.com/prod/GodWit"
license=('MIT' 'Apache-2.0')
depends=(
        # 'godwit-daemon'
         # 'weaver'
         )
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/Passeriform/GodWit/archive/v$pkgver.tar.gz")
optdepends=()
backup=()
options=()
install=
changelog=
md5sums=('4a8bf7e60a78e90b0824de2b31732f21') # Set by scripts/set-checksum

prepare() {
  # Fix naming inconsistency
  mv "GodWit-$pkgver" "$pkgname-$pkgver"
}

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release --all-features
}

check() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo test --release
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm 755 target/release/${pkgname} -t "${pkgdir}/usr/bin"
  chmod +x ./scripts/install-misc && ./scripts/install-misc ${pkgdir} ${pkgver}
}
