# Setheum Monorepo Makefile

.PHONY: all build test fmt headers clean

all: build

build:
	cargo build --release

test:
	cargo test --workspace --exclude setheum-js
	cd setheum-js && yarn install && yarn test

fmt:
	cargo fmt --all
	python3 scripts/apply_headers.py

headers:
	python3 scripts/apply_headers.py

clean:
	cargo clean
	rm -rf target/
