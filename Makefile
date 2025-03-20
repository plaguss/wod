sources = src

.PHONY: format
format:
	cargo fmt

.PHONY: lint
lint:
	cargo fmt --all -- --check
	cargo clippy --tests -- -D warnings

.PHONY: test-all
test-all:
	cargo test --lib
	cargo test --doc

.PHONY: install
install:
	cargo install --path .
