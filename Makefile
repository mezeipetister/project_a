.PHONY: release, test

release:
	cargo build --release
	strip target/release/project_a

build:
	cargo build

test:
	cargo test