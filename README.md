# Rust Wasmer simple plugin example

This repo modernizes the code from the [Using Wasmer for Plugins](https://freemasen.com/blog/wasmer-plugin-pt-1/index.html) tutorial series.

The series is over is from 2019 and many of the dependencies are out of date. There were only minor changes to `syn`, `quote`, and `proc_macro2` APIs (at least the parts used in the tutorial), but the original series uses the now-deprecated `wasmer_runtime` crate instead of `wasmer`.

Wasmer changed pretty significantly between version 0.3 and 4.0 (lol), so that was the main learning experience here. The commit history on master mostly follows each step from the tutorial linearly.

Other branches may contain random crap I tried out during/after the tutorial.

As a [contributor](https://github.com/wasmerio/wasmer/pull/4063) to the Wasmer project, I hope to eventually get comfortable interoping with wasm in rust


## Running the code

This branch uses a hardcoded path to the wasm binary, so if you run it, you'll have to make sure you have it there. Unless something changes, building the `example-plugin` crate with the wasm target should put it there.

```rust
cargo build -p example-plugin --target wasm32-unknown-unknown
cargo run -p example-runner
```
