run:
	bash -c "./build_run.sh"

build:
	cargo build --release

update:
	cargo update

clean:
	cargo clean

lint:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

format:
	cargo fmt --all