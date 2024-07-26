#!/bin/bash

cargo component build
wasmtime run -S http ./target/wasm32-wasi/debug/playground-wstd.wasm
