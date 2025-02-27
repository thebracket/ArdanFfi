# How about some Rust from C?

Consuming C (and libraries with a C interface) from Rust is generally pretty straightforward -- although we'll
look at some of the "ugly" cases in a bit. How about going the other way? You want to write some Rust,
and make use of it from another language.

> Note that Python has great support for calling Rust code. We'll touch on it at the end of the class, and hopefully it'll be its own class soon.