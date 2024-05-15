# Maintainer: Stephen Power <simpilldev@gmail.com>

pkgname=rustytube
pkgver=0.2.0
pkgrel=1
pkgdesc="A youtube client written in Rust using Leptos and Tauri; Designed with DaisyUI and Tailwind."
arch=('x86_64')
url="https://github.com/opensourcecheemsburgers/RustyTube"
license=('AGPL-3.0')
depends=('webkit2gtk' 'libayatana-appindicator')
makedepends=('npm' 'rustup' 'pkgconf')
source=("${pkgname}-${pkgver}.tar.gz::${url}/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('SKIP')

build() {
	cd $srcdir/${pkgname}-${pkgver}
	rustup update nightly-unknown-linux-gnu
	rustup component add rust-src --toolchain nightly-unknown-linux-gnu
    cargo install trunk
    cargo install tauri-cli
    rustup target add wasm32-unknown-unknown
    cd frontend
    npm install
	cd ..
	cargo tauri build -b none --target x86_64-unknown-linux-gnu -- -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
}

package() {
	cd $srcdir/${pkgname}-${pkgver}
	install -Dm755 target/x86_64-unknown-linux-gnu/release/${pkgname} -t ${pkgdir}/usr/bin
	install -Dm644 src-tauri/icons/icon.svg ${pkgdir}/usr/share/icons/hicolor/scalable/apps/${pkgname}.svg
	install -Dm644 ${srcdir}/${pkgname}.desktop -t ${pkgdir}/usr/share/applications
}
