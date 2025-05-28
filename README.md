# Tokay (WASM)

Tokay is a programming language designed for ad-hoc parsing.

This is the web-assembly NPM package of the [Tokay programming language](https://tokay.dev).

The Tokay programming language itself is developed in Rust, and can be found officially [here](https://github.com/tokay-lang/tokay).

## Build & publish

- Build:
  - for bundlers like `vite`: `wasm-pack build --target=bundler`
  - for nodejs: `wasm-pack build --target=nodejs`
  - for web: `wasm-pack build --target=web`
- Publish: `wasm-pack publish`

## License

Copyright © 2025 by Jan Max Meyer, Phorward Software Technologies.

Tokay is free software under the MIT license.<br>
Please see the LICENSE file for details.
