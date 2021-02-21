build:
	cargo build

test *args:
	cargo test --{{args}}

run:
	cargo run

fmt:
	cargo +nightly fmt
