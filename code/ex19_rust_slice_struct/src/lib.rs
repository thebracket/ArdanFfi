#[repr(C)]
#[derive(Debug)]
pub struct MyData {
    pub a: i32,
    pub b: i16,
    pub c: i8,
}

#[no_mangle]
pub extern "C" fn print_slice_of_mydata(ptr: *const u8, length: usize) {
    if ptr.is_null() || length == 0 {
        return;
    }
    assert_eq!(ptr.align_offset(std::mem::align_of::<MyData>()), 0, "Pointer is not aligned");    
    assert_eq!(length % std::mem::size_of::<MyData>(), 0, "Length is not a multiple of MyData size");
    let slice = unsafe { std::slice::from_raw_parts(ptr as *const MyData, length / size_of::<MyData>()) };
    for data in slice {
        println!("{data:?}");
    }
}

#[no_mangle]
pub extern "C" fn print_slice_nicely(ptr: *const MyData, num_elements: usize) {
    if ptr.is_null() || num_elements == 0 {
        return;
    }
    assert_eq!(ptr.align_offset(std::mem::align_of::<MyData>()), 0, "Pointer is not aligned");
    let slice = unsafe { std::slice::from_raw_parts(ptr, num_elements) };
    for data in slice {
        println!("{data:?}");
    }
}

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
