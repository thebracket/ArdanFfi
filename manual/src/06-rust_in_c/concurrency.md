# Concurrency!

We haven't really done much that shows Rust shining, yet. So let's fix that.

In `lib.rs`, let's build:

```rust
use rayon::prelude::*;

#[no_mangle]
pub extern "C" fn is_prime_slow(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..n/2+ 1 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[no_mangle]
pub extern "C" fn count_primes(slice: *const i32, len: usize) -> usize {
    // Safety, as much as we can
    assert!(!slice.is_null());
    assert!(len > 0);

    // Get the slice
    let slice = unsafe { std::slice::from_raw_parts(slice, len) };
    slice.par_iter().filter(|n| is_prime_slow(**n)).count()
}
```

And add Rayon to your dependencies with `cargo add rayon`. We've made a deliberately SLOW prime number detector, and then a function that uses Rayon to auto-parallelize it across all your CPUs and count the result.

Now for the C. Let's start by making sure it works:

```c
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>
#include <time.h>

#define ARR_SIZE 1000

// Link to the Rust versions
bool is_prime_slow(int32_t n);
int64_t count_primes(int32_t *arr, size_t len);

int main() {
    time_t start, end;

    // Allocate memory for the array
    printf("Allocating memory for the array...\n");
    int32_t *arr = (int32_t *)malloc(ARR_SIZE * sizeof(int32_t));

    // Populate the array with random numbers
    printf("Populating the array with random numbers...\n");
    for (int32_t i = 0; i < ARR_SIZE; i++) {
        arr[i] = rand();
    }

    // Count the primes in the array with Rust, cheating we're using Rayon
    start = time(NULL);
    int64_t sum_rust = count_primes(arr, ARR_SIZE);
    end = time(NULL);
    printf("Count (from Rust, Parallel): %ld. Seconds: %ld\n", sum_rust, end - start);

    // Free the allocated memory
    free(arr);
    return 0;
}
```


And here's a long version that tests all of it:

```c
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>
#include <time.h>

#define ARR_SIZE 1000

// Link to the Rust versions
bool is_prime_slow(int32_t n);
int64_t count_primes(int32_t *arr, size_t len);

// A native C version of the same thing
bool is_prime_slow_c(int32_t n) {
    if (n < 2) return false;
    for (int32_t i = 2; i < n/2; i++) {
        if (n % i == 0) return false;
    }
    return true;
}

int main() {
    time_t start, end;

    // Allocate memory for the array
    printf("Allocating memory for the array...\n");
    int32_t *arr = (int32_t *)malloc(ARR_SIZE * sizeof(int32_t));

    // Populate the array with random numbers
    printf("Populating the array with random numbers...\n");
    for (int32_t i = 0; i < ARR_SIZE; i++) {
        arr[i] = rand();
    }

    // Count the primes in the array with Rust, cheating we're using Rayon
    start = time(NULL);
    int64_t sum_rust = count_primes(arr, ARR_SIZE);
    end = time(NULL);
    printf("Count (from Rust, Parallel): %ld. Seconds: %ld\n", sum_rust, end - start);

    // Count the primes in the array with a C loop
    printf("Calculating the sum of the array in C...\n");
    start = time(NULL);
    int64_t sum = 0;
    for (int32_t i = 0; i < ARR_SIZE; i++) {
        if (is_prime_slow(arr[i])) sum++;
    }
    end = time(NULL);
    printf("Count (from C): %ld. Seconds: %ld\n", sum, end - start);

    // Count the primes in the array with a C loop using native code
    printf("Calculating the sum of the array in C (native)...\n");
    start = time(NULL);
    sum = 0;
    for (int32_t i = 0; i < ARR_SIZE; i++) {
        if (is_prime_slow_c(arr[i])) sum++;
    }
    end = time(NULL);
    printf("Count (from native C): %ld. Seconds: %ld\n", sum, end - start);

    // Free the allocated memory
    free(arr);
    return 0;
}
```

On my system at the office:

* Rust completes in 4 seconds.
* A C loop to the Rust function completes in 33 seconds.
* A C loop to the native C function completes in 33 seconds.

So we've learned: 
* Rayon makes it easy to get a big win if you have multiple CPUs - and Rust makes concurrency much less scary.
* There really isn't a performance penalty for calling into Rust.
