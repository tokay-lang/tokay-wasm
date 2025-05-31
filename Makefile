SHELL = /bin/bash
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := build
.DELETE_ON_ERROR:
.SUFFIXES:

build: pkg

pkg: src
	wasm-pack build --out-dir=pkg --target=bundler --out-name=tokay
	sed -i -e 's/"name": "tokay-wasm"/"name": "tokay"/g' pkg/package.json

publish:
	wasm-pack publish

clean:
	cargo clean
	rm -rf pkg-bundler
