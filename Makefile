sources = src

.PHONY: format
format:
	cargo fmt

.PHONY: lint
lint:
	cargo fmt --all -- --check
	cargo clippy --tests -- -D warnings

.PHONY: test
test:
	cargo test
