SHELL := /bin/bash
TARGET := x86_64-unknown-linux-musl

.PHONY: all build-static audit test clean

all: test audit build-static

test:
	cargo test --release

audit:
	cargo clippy --all-targets -- -D warnings -D clippy::pedantic -D clippy::unwrap_used

build-static:
	@echo "Construyendo binario estatico hermetico con musl..."
	cargo build --release --target $(TARGET)
	@sha256sum target/$(TARGET)/release/auraintel-expansion-engine > SHA256SUMS-BUILD.txt
	@echo "Artefacto generado y sellado con SHA-256."

clean:
	cargo clean
