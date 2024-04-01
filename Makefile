test:
	cargo test

run:
	cargo run

update:
	rustup update

int:
	cargo init

build:
	cargo build --release

.PHONY: test run update int
