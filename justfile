build:
	cargo build

test *args:
	cargo test -- --{{args}}

run *args:
	cargo run {{args}}

fmt:
	cargo +nightly fmt
