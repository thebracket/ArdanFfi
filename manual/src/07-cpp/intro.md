# C++

**First of all**: *Everything* we've learned about C also works with C++, with the exception of remembering the `extern C` blocks in C++.

> C++ started as "C with classes" in **1983**! The first C++ standard launched in 1998, and the standard library as we know it today appeared in 2005. In 2011, C++11 added working smart pointers. Containers added safe range-checked accessors, strings gained a safe wrapper, and we haven't had any memory errors since.

Snarkiness aside, we --- the Rust community --- owe a lot to C++. If you run into Mr. Stroustrup, shake his hand and be nice!

In particular:

|**Feature**|**Rust Feature**|
|--|--|
|RAII|`Drop`|
|`unique_ptr`|`Box` - it's basically the same thing|
|`shared_ptr`|`Arc` and `Rc`|
|`std::string`|`string`|
|`std::vector`|`vec`|
|Exceptions|Ok, we dodged that bullet|
|Terrifying template meta-programming|Often-terrifying Generics|

The C++ world has been in a bit of a panic, at least in America. The Department of Defense declared that new code has to be created in a memory safe language --- and a lot of defence contractors have legacy C++ code that's nearly as old as I am. It's not at all uncommon to go to train a company who have *millions* of lines of C++, gradually accumulated (and gradually adopting new language features, sometimes) over the decades.

So when you go into these companies, you face:

* The team may not actually want to learn Rust. In fact, there's quite a bit of open hostility. `r/cpp` on Reddit says some really mean things about us. (We can help by not saying "rewrite it in Rust" quite so often).
* There's always *That One Guy* who has been writing C++ since the time of the dinosaurs and is going to make your life as miserable as possible. Witness the Rust for Linux team facing a long-time Linux contributor screaming "YOU CAN'T MAKE ME LEARN RUST!" while the presenter tried to explain that they weren't asking him to.
* Grief goes through Denial, Anger, Bargaining, Depression and Acceptance (Kübler-Ross). Sudden career-path changes are hard, it's natural. Unfortunately, sometimes the Rust community is on the wrong end of the processing chain---but be a good human and help!

A fellow named Sean Baxter recently proposed a "Safe C++" that implements:

* A borrow checker.
* Range-checked containers by default.
* An `unsafe` tag, and requiring its use for naked pointer operations.

Sadly, the C++ standards committee reacted *very* badly. So Mr. Baxter came up with a second proposal that would tie C++ and Rust together, ensuring a baseline level of compatibility. Unfortunately, that went down so well that Mr. Baxter has changed careers.