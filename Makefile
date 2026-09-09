.PHONY: fmt fmt-check lint test build build-release local-install

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

build:
	cargo build --workspace --all-targets

build-release:
	cargo build --workspace --release --all-targets

local-install:
	cargo install --path .
