install: build
	cargo install --bin gnostr-server --path .
build:
	@cargo build

clean:
	@cargo clean

TEST = "curl_test"
test:
	@cargo test $(TEST) --offline       -- --color=always --nocapture || \
		cargo test --tests
tests:
	@cargo test --tests --offline       -- --color=always --nocapture

docs: lint build
	@cargo doc --no-deps

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --all-targets --all-features -- -D warnings

dev:
	@cargo run -q --bin gnostr-server

run:
	@cargo -q run -- --port 8080
run-verbose:
	@cargo -q run -- -vv --port 8080

.PHONY: build test docs style-check lint
