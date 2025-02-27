# Interop Would be Nice

> I promise not to sing it, but Imagine by John Lennon should accompany this slide.

In an ideal world, your C++ and Rust code would work really nicely together. C++ classes would be readily accessible from Rust, and complex Rust types would be readily available from C++. The types that are basically *the same* would work together. This is an ongoing area of work for the Rust Foundation, and is greatly in-demand.

LLVM (and GCC) compiles both Rust and C++. Yet there is no common way to link them together, other than the C ABI. You can do a *lot* with that, but you lose a fair amount of the richness of both languages. It's two highly sophisticated individuals talking through a string telephone.

Unfortunately, we're not quite there yet. It's getting better.
