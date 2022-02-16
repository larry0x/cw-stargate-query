#!/usr/bin/make -f

OPTIMIZER_VERSION = "0.12.5"

build: gen-rust
	@echo "--> Compiling WASM contract"
	@cd contract
	@docker run --rm -v "$(shell pwd)/contract":/code \
		--mount type=volume,source="$(shell basename $(shell pwd))_cache",target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:$(OPTIMIZER_VERSION)
	@cd ..
	@mv contract/artifacts .

gen-rust:
	@echo "--> Generating Rust code from Protobuf"
	@bash scripts/proto_rust_gen.sh