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