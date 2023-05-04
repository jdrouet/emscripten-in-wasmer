# Emscripten in Wasmer

Just an attempt to use a library built with emscripten in wasmer.

## How to make it "work"

1. Build the `magic-lib`

```bash
cd magic-lib
wasm-pack build
cd ..
```

2. Run the server

```bash
cd magic-server
cargo run
cd ..
```