.PHONY: fmt build install

fmt:
	cargo fmt
	cargo fix

build:
	cargo build --release

install:
	make build
	mkdir -p "$HOME/.local/bin"
	cp -v ./target/release/subshift "$HOME/.local/bin"
