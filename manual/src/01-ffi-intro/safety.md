# FFI and Safety


You're going to type `unsafe` a lot more than you're (hopefully) used to! 
There's even been a proposal for a "safe unsafe" flag!

> The "safe unsafe" tag is horribly named, but the concept makes some sense. FFI involves a lot of "unsafe" tags, because you are stepping beyond what Rust can verify - rather than doing something actually unsafe. "Safe Unsafe" would be a way to tag that while this operation can't be verified, it has been extensively checked. 

## The Types of Unsafe

The `unsafe` tag isn't inherently bad. It's a way to tell the compiler that you're doing something
that cannot be verified by Rust's safety rules. FFI - calling code outside of Rust - is inherently
"unsafe" because Rust can't reach into the external code and verify that it, too, is safe.

You tend to run into a few types of "unsafe" code:

1. **External Code**: you're calling code that Rust cannot verify.
2. **Dangerous Pointers**: you're interacting with pointers in a way that Rust can't verify. For example, linked lists. You run into this inside many libraries. The `unsafe` tag serves as a marker---"check here".
3. **But It's Faster**: you're going for an optimization that Rust can't verify as safe. Don't do this unless you *really* have no alternative, and have profiled extensively.
4. **YOLO**: You Only Live Once, and really want to do something. Don't do this.

## What's Unsafe?

In the 2024 edition, Rust has marked a few more things "unsafe". The class uses the Rust 2024 edition;
if you're running an older compiler, you'll need to run `rustup update` to get the latest edition.

So... let's write some horribly unsafe code!

![](../images/crab-leroy.webp)

> I promise, it's not really that unsafe!