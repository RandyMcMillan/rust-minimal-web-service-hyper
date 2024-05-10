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

docs: build
	@cargo doc --no-deps

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --all-targets --all-features -- -D warnings

dev:
	@cargo run -q

run:
	@cargo -q run -- --port 3000
run-verbose:
	@cargo -q run -- -vv --port 3000

.PHONY: build test docs style-check lint
