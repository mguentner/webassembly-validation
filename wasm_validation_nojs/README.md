validation library targeted for non-javascript wasm runtimes such as wazero

build with

```
cargo build --release --target wasm32-unknown-unknown
```

then copy / distribute the artifact `target/wasm32-unknown-unknown/release/validation_nojs.wasm`

# License

MIT

Allocator and Deallocator are taken from [tetratelabs/wazero](https://github.com/tetratelabs/wazero/blob/main/examples/allocation/rust/testdata/greet.rs)