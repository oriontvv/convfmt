
.PHONY: debug build-release release-linux-musl test clippy clippy-pedantic install install-debug

PROJECT=convfmt

ARGS=-l

profile:
	cargo run --features=timing,pprof -- ${ARGS}

run-timing:
	cargo run --features=timing --release -- ${ARGS}

debug:
	RUST_BACKTRACE=true cargo run --features=timing -- ${ARGS}

build-release:
	cargo build --release

release-mac: build-release
	strip target/release/${PROJECT}
	otool -L target/release/${PROJECT}
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/${PROJECT}-mac.tar.gz ./${PROJECT}
	ls -lisah ./release/${PROJECT}-mac.tar.gz

release-win: build-release
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/${PROJECT}-win.tar.gz ./${PROJECT}.exe
	cargo install cargo-wix --version 0.3.3
	cargo wix --no-build --nocapture --output ./release/${PROJECT}.msi
	ls -l ./release/${PROJECT}.msi 

release-linux-musl: build-linux-musl-release
	strip target/x86_64-unknown-linux-musl/release/${PROJECT}
	mkdir -p release
	tar -C ./target/x86_64-unknown-linux-musl/release/ -czvf ./release/${PROJECT}-linux-musl.tar.gz ./${PROJECT}

build-linux-musl-debug:
	cargo build --target=x86_64-unknown-linux-musl

build-linux-musl-release:
	cargo build --release --target=x86_64-unknown-linux-musl

test-linux-musl:
	cargo test --workspace --target=x86_64-unknown-linux-musl

test:
	cargo test --workspace

fmt:
	cargo fmt -- --check

clippy:
	cargo clippy --workspace --all-features

clippy-nightly:
	cargo +nightly clippy --workspace --all-features

check: fmt clippy test

install:
	cargo install --path "." --offline

install-timing:
	cargo install --features=timing --path "." --offline

licenses:
	cargo bundle-licenses --format toml --output THIRDPARTY.toml
