# Let's Try It!

Create a new project (`cargo new ex21 --lib`). The `Cargo.toml` looks like this:

```toml
[package]
name = "ex21_async"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["staticlib"] # Will create a .so on Linux, .dylib on Mac, .dll on Windows

[dependencies]
tokio = { version = "1.43.0", features = ["full"] }
```

Let's get started on `lib.rs`. Some superstructure:

```rust
use std::{sync::OnceLock, thread};
use tokio::sync::oneshot;

enum Command {
    AsyncGenerator { callback: extern "C" fn(i32, i32), complete: oneshot::Sender<()>, n: i32 },
}

static COMMAND_TX: OnceLock<tokio::sync::mpsc::Sender<Command>> = OnceLock::new();
```

We're making an enum containing the commands we want to be able to pass into Tokio-land. We've also created a `OnceLock` that will contain a channel sender.

Now we can make a function to start Tokio:

```rust
#[no_mangle]
pub extern "C" fn start_generator() {
    // Oneshot: so we know when Tokio is alive
    let (tx, rx) = oneshot::channel();

    // Command channel
    let (cmd_tx, mut cmd_rx) = tokio::sync::mpsc::channel(100);
    COMMAND_TX.set(cmd_tx).unwrap();

    // In a thread, so that thread "blocks on" forever...
    thread::spawn(move || {
        // Start Tokio runtime
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Tokio is now alive!
            tx.send(()).unwrap();

            // Process commands
            while let Some(cmd) = cmd_rx.recv().await {
                match cmd {
                    Command::AsyncGenerator { callback, complete, n } => {
                        tokio::spawn(generator(callback, complete, n));
                    }
                }
            }
        });

        // Wait for the response
        let _ = rx.blocking_recv().unwrap();
    });
}
```

Let's build the generator function:

```rust
async fn generator(callback: extern "C" fn(i32, i32), complete: oneshot::Sender<()>, n: i32) {
    for i in 0..10 {
        // Simulate async work
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        // Call the callback
        callback(i, n);
    }

    // Send the result back to the main thread
    complete.send(()).unwrap();
}
```

And finally, an interface function to allow it to be called externally:

```rust
#[no_mangle]
pub extern "C" fn async_generator(callback: extern "C" fn(i32, i32), n: i32) {
    // Oneshot: so we know when the generator is done
    let (tx, rx) = oneshot::channel();

    // Send the command to Tokio
    let _ = COMMAND_TX
        .get()
        .unwrap()
        .blocking_send(Command::AsyncGenerator {
            callback,
            complete: tx,
            n,
        });

    // Wait for the response
    let _ = rx.blocking_recv().unwrap();
}
```

Now let's make a C directory. We're going to use C++ (so pthreads don't drive us insane). Here's the build script:

```bash
#!/bin/bash
CARGO_TARGET_DIR="tmp" cargo build
cp tmp/debug/libex21_async.a .
CARGO_TARGET_DIR="tmp" cargo clean
c++ -std=c++17 ex21.cc -o ex21 libex21_async.a
./ex21
```

And here's the C++ file `ex21.cc`:

```cpp
#include <thread>
#include <vector>
#include <stdio.h>

// Notice that C++ requires the "extern "C" - it has name
// mangling, too.
extern "C" {
    void start_generator();
    void async_generator(void (*callback)(int, int), int n);
}

void thread_function(int i) {
    async_generator([](int generated, int thread_id) {
        printf("[%d] Called with %d\n", thread_id, generated);
    }, i);
}

int main() {
    // Launch the async Rust
    start_generator();

    // Create and start threads
    std::thread t1(thread_function, 1);
    std::thread t2(thread_function, 2);

    // Wait for all threads to finish
    t1.join();
    t2.join();
    
    return 0;
}
```

If you run this, you'll see that the threads are calling into the async runtime and values are being yielded. It's super-efficient, because the threads are put to sleep by the channel call (on the Rust side), and everything wakes up as needed.

A simple arithmetic generator isn't all that useful, but if you have Rust async code that uses databases, the network, or other naturally async properties---you can now link it into your C or C++.