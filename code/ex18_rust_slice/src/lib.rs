#[no_mangle]
pub extern "C" fn sum_byte_slice(ptr: *const u8, length: usize) -> i32 {
    if ptr.is_null() || length == 0 {
        return 0;
    }
    let slice = unsafe { std::slice::from_raw_parts(ptr, length) };
    let mut sum: i32 = 0;
    for &num in slice {
        sum += num as i32;
    }
    sum
}