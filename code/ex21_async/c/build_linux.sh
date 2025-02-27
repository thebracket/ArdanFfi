#!/bin/bash
CARGO_TARGET_DIR="tmp" cargo build
cp tmp/debug/libex21_async.a .
CARGO_TARGET_DIR="tmp" cargo clean
c++ -std=c++17 ex21.cc -o ex21 libex21_async.a
./ex21
