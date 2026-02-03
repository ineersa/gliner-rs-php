IMAGE_NAME ?= gliner-rs-php
STUBS_FILE ?= gliner_stubs.php

.PHONY: help docker-build docker-rebuild docker-test cargo-build cargo-test cargo-clean cargo-stubs

help:
	@printf "Targets:\n"
	@printf "  docker-build    Build Docker image (IMAGE_NAME=%s)\n" "$(IMAGE_NAME)"
	@printf "  docker-rebuild  Rebuild Docker image without cache\n"
	@printf "  docker-test     Build image and run PHP test.php\n"
	@printf "  cargo-build     Build the Rust crate\n"
	@printf "  cargo-test      Run test suite\n"
	@printf "  cargo-clean     Remove Cargo build artifacts\n"
	@printf "  cargo-stubs     Generate PHP stubs (STUBS_FILE=%s)\n" "$(STUBS_FILE)"

docker-build:
	docker build -t $(IMAGE_NAME) .

docker-rebuild:
	docker build --no-cache -t $(IMAGE_NAME) .

docker-test: docker-build
	docker run --rm $(IMAGE_NAME)

cargo-build:
	cargo build

cargo-test:
	RUSTFLAGS="-L/usr/lib -lphp8.4" cargo test

cargo-clean:
	cargo clean

cargo-stubs:
	@command -v cargo-php >/dev/null 2>&1 || { \
		printf "cargo-php is required for stub generation. Install with: cargo install cargo-php --locked\n"; \
		exit 1; \
	}
	cargo php stubs --stdout > $(STUBS_FILE)