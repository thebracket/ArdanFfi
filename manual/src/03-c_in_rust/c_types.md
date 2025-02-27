# C Types

C is a simple language, just like a high-level assembly, right? Let's talk a bit about C types - and continue to port some code.

## Pop Quiz

The C type `int`. Raise your hand if you beleve it means:

A signed integer.

![](../images/ScrollTime.png)

A signed 32-bit integer.


![](../images/ScrollTime.png)

A type that is at least 16-bits wide, but the actual width is implementation defined. It is guaranteed to be at least as large as a `short int`, is not guaranteed to be smalller than a `long` and is frequently, but not always, the native bit-size of your target platform.

> This a a trick question, because the C standard has been adding requirements to integers. When C first came along, it didn't *have* to be signed - because not every platform supported it. The most recent C++ standard even requires two's complement encoding.

## C Types

* signed char
* short int (or short)
* int
* long int (or long)
* long long int (or long long)
* unsigned char
* unsigned short int (or unsigned short)
* unsigned int
* unsigned long int (or unsigned long)
* unsigned long long int (or unsigned long long)
* float
* double
* long double
* char (can be either signed or unsigned, depending on the implementation, but is a distinct type from signed char and unsigned char).
* void

With the exception of `void` (and `char` in the most recent C standard) - *all* of these are platform dependent. This is mostly because C has been around for a while!

For example:

* The TI DSP TMS320 uses a *16 bit* `char`!
* The old CDC 6000 series had `char` available in 6, 9 and 12 bit types.
* You really should check `CHAR_BIT` in `<limits.h>` for exotic platforms.