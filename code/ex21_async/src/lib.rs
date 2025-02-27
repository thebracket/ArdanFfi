use std::{sync::OnceLock, thread};
use tokio::sync::oneshot;

enum Command {
    AsyncGenerator { callback: extern "C" fn(i32, i32), complete: oneshot::Sender<()>, n: i32 },
}

static COMMAND_TX: OnceLock<tokio::sync::mpsc::Sender<Command>> = OnceLock::new();

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