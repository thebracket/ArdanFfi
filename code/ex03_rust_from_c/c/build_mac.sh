#!/bin/bash
mkdir -p tmp
CARGO_TARGET_DIR="tmp" cargo build
cp tmp/debug/libex03_rust_from_c.dylib .
CARGO_TARGET_DIR="tmp" cargo clean
cc rust_from_c.c -o rust_from_c -L . -l ex03_rust_from_c
DYLD_LIBRARY_PATH=. ./rust_from_c

