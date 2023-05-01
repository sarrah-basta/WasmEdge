### Code 

## CPP
A new plugin called GetString with module name "wasi_string_test" is created in the folder `plugin/string_test`.

To build WasmEdge with this plugin, use the commands : 
```bash
mkdir -p build && cd build
cmake -DCMAKE_BUILD_TYPE=Release -DWASMEDGE_PLUGIN_STRING_TEST=On .. && make -j8 
cmake --install . 
```

## Rust
The function return_obtained_string of this plugin is then used in the rust code situated in the folder `./rust`. 

Run the commands : 

```bash
cargo build --target=wasm32-wasi --release
wasmedgec target/wasm32-wasi/release/rust.wasm out.wasm
wasmedge --dir .:. out.wasm
```
