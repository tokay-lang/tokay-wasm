# Tokay (WASM)

This is the web-assembly package of the [Tokay programming language](https://tokay.dev) for NPM.

Tokay is a programming language designed for ad-hoc parsing. It is written in Rust, and can be found officially [here](https://github.com/tokay-lang/tokay).

> [!IMPORTANT]
> Tokay (WASM) is in an early project state, therefore things on the package as well as the bindings may change!
> The package is kept synchronous to the current Tokay release version.
>

## Usage

```js
import { run } from 'tokay';

console.log(run("Int * 3", "123 456 789"));
// (369, 1368, 2367)
```

## Build & publish

Althought this crate and repo is called `tokay-wasm`, the released npm package is called just `tokay`.
This is ensured by the `Makefile`. Run `make pkg` and `make publish` to perform the required steps.

Invoking manual builds:

- for bundlers like `vite`: `wasm-pack build --target=bundler --out-name=tokay`
- for nodejs: `wasm-pack build --target=nodejs --out-name=tokay`
- for web: `wasm-pack build --target=web --out-name=tokay`

## License

Copyright © 2025 by Jan Max Meyer, Phorward Software Technologies.

Tokay is free software under the MIT license.<br>
Please see the LICENSE file for details.
