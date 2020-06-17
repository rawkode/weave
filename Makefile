.PHONY: build clean

build:
	@cargo build

run:
	@RUST_LOG=debug cargo run -- build -d .

clean:
	@cargo clean
