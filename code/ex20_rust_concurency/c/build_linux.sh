#!/bin/bash
#!/bin/bash
CARGO_TARGET_DIR="tmp" cargo build --release
cp tmp/release/libex20_rust_concurency.a .
CARGO_TARGET_DIR="tmp" cargo clean
cc -O3 ex20.c -o ex20 libex20_rust_concurency.a
cc -O3 ex20_native_threads.c -o ex20_native_threads -lpthread
./ex20
