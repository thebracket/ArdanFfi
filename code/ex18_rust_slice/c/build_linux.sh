#!/bin/bash
#!/bin/bash
CARGO_TARGET_DIR="tmp" cargo build
cp tmp/debug/libex18_rust_slice.a .
CARGO_TARGET_DIR="tmp" cargo clean
cc ex18.c -o ex18 libex18_rust_slice.a
./ex18
