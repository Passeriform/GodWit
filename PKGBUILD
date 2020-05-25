# Maintainer: Utkarsh Bhardwaj (Passeriform) <passeriform.ub@gmail.com>
pkgname=godwit
pkgver=0.1.0
pkgrel=1
pkgdesc="A hackable yet sane project manager and automation suite."
arch=('x86_64' 'i686' 'armv6h' 'armv7h')
url="https://www.passeriform.com/prod/GodWit"
license=('MIT OR Apache')
depends=('godwit-daemon'
         'weaver')
makedepends=('rust' 'cargo')
source=("$pkgname-v$pkgver::https://github.com/Passeriform/GodWit/archive/v$pkgver.tar.gz")
optdepends=()
backup=()
options=()
install=
changelog=
md5sums=('c87d5c9d6282089d5f91a2dd81205b4c')

check() {
  cargo test --release --locked
}

build() {
  cargo doc
  cargo build --release --locked --all-features
  cargo post-build
}

package() {
  install -Dm 755 target/release/${pkgname} -t "${pkgdir}/usr/bin"
}
