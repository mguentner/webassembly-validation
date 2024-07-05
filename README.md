# WebAssembly Validation

*build once, validate everywhere*

Blog post: https://sourcediver.org/posts/240628_unified_validation_with_webassembly

## Build and run

Execute in this order.

### Rust Backend

```
cd backend
cargo run
```

You can now send requests to the frontend. Checkout `requests` for [bruno files](https://www.usebruno.com/).

### wasm_validation

You need to have [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed.
A `flake.nix` is provided.

```
cd wasm_validation
wasm-pack build
```

### React Frontend

You need `pnpm` installed. A `flake.nix` has been provided.

```
pnpm install
pnpm dev
```

### wasm_validation_nojs

You need to have rustc with wasm32 support installed. A `flake.nix` is provided.

```
cd wasm_validation_nojs
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/validation_nojs.wasm ../go_client/. 
```

### go_client

You need a go compiler.
Make sure that you have copied the wasm file in the previous step!

```
cd go_client
go build
./go_client
```
# Feedback 

Feedback is welcome. Open an issue or pull request.

# License

MIT Maximilian Güntner 2024