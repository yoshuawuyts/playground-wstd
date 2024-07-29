#!/bin/bash

cargo component build
wasmtime run -S http ./target/wasm32-wasip1/debug/playground-wstd.wasm
