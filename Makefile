build:
	cargo build --release

lint:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

format:
	cargo fmt --all