#!/bin/bash
#!/bin/bash
CARGO_TARGET_DIR="tmp" cargo build
cp tmp/debug/libex19_rust_slice_struct.a .
CARGO_TARGET_DIR="tmp" cargo clean
cc ex19.c -o ex19 libex19_rust_slice_struct.a
./ex19
