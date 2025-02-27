# Working with Rust in C

We're not going to heavily belabor this, but I'd like to make sure you're equipped to offer your awesome Rust to other languages. Except for languages with nifty binding systems (Python in particular, NodeJS too)---you're going to need to use the C ABI.

Rust *doesn't have* a stable ABI. At all. It's not even guaranteed that a binary will be exactly the same if you compile it twice on the same system. So you *may* even find yourself using a C ABI in Rust if you want to support a plugin system. 

> You can also pay for Gitoxide and have a stable ABI, eventually! You won't be updating your Rust version very often, but it's possible.
