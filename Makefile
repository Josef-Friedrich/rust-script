test:
	cargo test

run:
	cargo run

update:
	rustup update

init:
	cargo init

build:
	cargo build --release

install:
	cargo install --force --path .

.PHONY: test run update init
