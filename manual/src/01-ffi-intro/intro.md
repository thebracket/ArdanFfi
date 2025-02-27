# Getting Started with FFI

FFI stands for "Foreign Function Interface". While that sounds rather alarming, it just means "stuff that wasn't written in Rust".

The "C ABI" --- Application Binary Interface, we're playing acronym soup --- is the lingua franca of modern operating systems. It's sometimes the only well-defined binary interface for calling functions and exchanging data. Unfortunately, you can't just drop a Rust library straight into a C++, Go, C#, etc. project and expect it to do much. The languages are different, and need a common language in order to communicate.

FFI is that commonality.

Rust is great at FFI, it was one of the original design goals. There's no performance penalty (like CGo, C# Marshaling, etc.), but you do lose some of the richness of the type system.