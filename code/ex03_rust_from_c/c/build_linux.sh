#!/bin/bash
mkdir -p tmp
CARGO_TARGET_DIR="tmp" cargo build
cp tmp/debug/libex03_rust_from_c.so .
CARGO_TARGET_DIR="tmp" cargo clean
cc rust_from_c.c -o rust_from_c -L. -lex03_rust_from_c
LD_LIBRARY_PATH=. ./rust_from_c
