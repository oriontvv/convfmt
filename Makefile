
.PHONY: debug build-release release-linux-musl test clippy clippy-pedantic install install-debug web-tools web-build web-test web-serve docker-build docker-run

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

# web version: wasm build of the library plus the static page around it.
# wasm-bindgen-cli has to match the locked wasm-bindgen crate exactly,
# so the version is taken from web/Cargo.lock instead of being duplicated.
WASM_BINDGEN_VERSION = $(shell awk '/^name = "wasm-bindgen"$$/ { found = 1; next } found && /^version/ { gsub(/"/, "", $$3); print $$3; exit }' web/Cargo.lock)

web-tools:
	rustup target add wasm32-unknown-unknown
	cargo binstall -y wasm-bindgen-cli@$(WASM_BINDGEN_VERSION)

web-build:
	cd web && cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen web/target/wasm32-unknown-unknown/release/convfmt_web.wasm \
		--target web --no-typescript --out-dir web/static/pkg

web-test:
	cd web && cargo test

web-serve: web-build
	python3 -m http.server 8000 --directory web/static

# local image for the current platform; ci builds the multiarch one, see release.yml
DOCKER_IMAGE = oriontvv/${PROJECT}

docker-build:
	docker build -t $(DOCKER_IMAGE):dev .

docker-run: docker-build
	docker run --rm -i $(DOCKER_IMAGE):dev ${ARGS}

coverage:
	rustup component add llvm-tools-preview
	cargo install grcov
	mkdir -p target/coverage
	export RUSTFLAGS="-Cinstrument-coverage"
	cargo build
	LLVM_PROFILE_FILE='target/coverage/%p-%m.profraw' RUSTFLAGS='-C instrument-coverage' cargo test
	grcov . -s . --binary-path ./target/debug --ignore src/main.rs -t html --branch --ignore-not-existing -o ./htmlcov/
	# open ./htmlcov/index.html

fmt:
	cargo fmt -- --check

clippy:
	cargo clippy --workspace --all-features

lint: fmt clippy

clippy-nightly:
	cargo +nightly clippy --workspace --all-features

check: fmt clippy test

install:
	cargo install --path "." --offline

install-timing:
	cargo install --features=timing --path "." --offline

licenses:
	cargo bundle-licenses --format toml --output THIRDPARTY.toml
