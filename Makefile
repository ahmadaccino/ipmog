.PHONY: build install run clean

build:
	cargo build --release

install-on-machine: build
	mkdir -p ~/.local/bin
	cp target/release/ipmog ~/.local/bin/ipmog

run:
	cargo run --release

clean:
	cargo clean
