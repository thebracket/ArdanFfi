#!/bin/bash
CARGO_TARGET_DIR="tmp" cargo build
cp tmp/debug/libex03a_rust_from_c_static.a .
CARGO_TARGET_DIR="tmp" cargo clean
cc rust_from_c.c -o rust_from_c_static libex03a_rust_from_c_static.a
./rust_from_c_static
