# Rust 🦀 + WebAssembly 📦 = 💝

This is a personal playground for my experimentations with Rust and WebAssembly (using [wasm-pack](https://github.com/rustwasm/wasm-pack) & [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen)).

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) 🤷
- [wasm-pack](https://github.com/rustwasm/wasm-pack)
- And, depending on the target platform:
  - Node.js 8+, **or**,
  - A [compatible](https://caniuse.com/#search=webassembly) browser

## Getting started

Clone the repository and execute `wasm-pack build` in order to generate the WebAssembly binary and its JavaScript binding as a npm module in `./pkg`. The default bindings are supposed to be consumed by a bundler like [Webpack](https://webpack.js.org/). Use `--target` to build a module for **Node.js**: `wasm-pack build --target nodejs`.

> 💡 Add the parameter `--release` to build an optimized module.

The resulting module is already installed as a dependency (`rust-wasm-introduction` from **`./pkg`**) in the two TypeScript projects: `./nodejs` and `./browser`. Thus, it can be imported as any regular module.

## Node.js & Browser

The Wasm module is available in the two projects. Make sure you built it using the valid `target`, depending on the project you want to use.

Use the scripts available to `build`, `test` and `start` the projects (see their respective `package.json`).

The function `naive_fibonacci` has been written twice, once in Rust, once in JavaScript, to make a basic comparison of their performance in Node.js. See `./nodejs/src/naive-fibonacci`.

## Adding a Rust package

Consider creating a new library when you need to add a package to the output.

```shell
$ cargo new --lib --name new-fancy-lib packages/new-fancy-lib
     Created library `new-fancy-lib` package
```

Add `wasm-bingden` to its dependencies:

```toml
# packages/new-fancy-lib/Cargo.toml
[dependencies]
wasm-bindgen = "0.2"
```

Then declare it as a dependency of the main package:

```toml
# Cargo.toml
[dependencies]
new-fancy-lib = { version = "0.1.0", path = "packages/new-fancy-lib" }
```

And finally, export it from `src/lib.rs`.

```rust
pub use new_fancy_lib;
```

Compile the project again (`wasm-pack build`) to see the new JavaScript bindings from `new-fancy-lib` 🎉.

## Testing a Rust package

Simply run `cargo test -p` with the name of the package.

```shell
$ cargo test -p bank-account
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running target/debug/deps/bank_account-87bf4d4300aefea4

running 3 tests
test tests::it_deposits_the_given_amount ... ok
test tests::it_withdraws_the_given_amount ... ok
test tests::test_negative_balance_on_withdraw ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

💡 At the present time, I have no particular interest in using `wasm_bindgen_test` (and running tests with `wasm-pack`), but this might change in the future.
