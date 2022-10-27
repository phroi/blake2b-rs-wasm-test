# blake2b-rs wasm test

This test is loosely based on the [Mozilla's guide: Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)

To run these tests:

- Install Rust and Clang then
- `cargo install wasm-pack;`
- `cargo build; //Ok`
- `cargo build --target wasm32-unknown-unknown; //fatal error: 'string.h' file not found`

This error is related to: [Status of rust+C bindings for wasm targets](https://github.com/rustwasm/team/issues/291)
