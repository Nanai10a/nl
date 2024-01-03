#!/bin/sh
rustc main.rs --target wasm32-wasi && wat2wasm entry.wat && wasmtime run --preload env=entry.wasm main.wasm
