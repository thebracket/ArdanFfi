mod c_lib {
    include!(concat!(env!("OUT_DIR"), "/clib.rs"));
}

mod rs {
    use std::ffi::c_char;

    pub fn double_byte(n: c_char) -> c_char { n.wrapping_mul(2) }
}

pub use rs::*;

#[cfg(test)]
mod tests {
    use std::{ffi::CStr, os::raw::{c_char, c_void}};
    use super::c_lib::*;

    #[test]
    fn test_double() {
        unsafe {
            assert_eq!(double_byte(2), 4);
        }
        assert_eq!(super::rs::double_byte(2), 4);
    }

    #[test]
    fn range_test_double() {
        for n in c_char::MIN .. c_char::MAX {
            let c_result = unsafe { double_byte(n) };
            let rust_result = super::rs::double_byte(n);
            assert_eq!(c_result, rust_result);
        }
    }

    #[test]
    fn test_copy_struct() {
        let s = MyStruct { integer: 12, byte: 3 };
        let n = unsafe { copy_struct(s) };
        assert_eq!(n, s.integer);
    }

    #[test]
    fn test_return_struct() {
        let s = unsafe { return_struct(11, 2) };
        assert_eq!(s.byte, 2);
        assert_eq!(s.integer, 11);
    }

    #[test]
    fn test_reference_struct() {
        let mut s = MyStruct { integer: 9, byte: 7 };
        let n = unsafe { reference_struct(&mut s) };
        assert_eq!(n, 9);
    }

    #[test]
    fn test_reference_struct_const() {
        let s = MyStruct { integer: 9, byte: 7 };
        let n = unsafe { reference_struct_const(&s) };
        assert_eq!(n, 9);
    }

    #[test]
    fn test_is_byte_twelve() {
        let s = MyStruct { integer: 0, byte: 12 };
        let mut result = false; // Rust won't let you not initialize it
        let retval = unsafe { is_byte_twelve(&s, &mut result) };
        assert_eq!(0, retval);
        assert_eq!(true, result);

        let s = MyStruct { integer: 0, byte: 13 };
        let retval = unsafe { is_byte_twelve(&s, &mut result) };
        assert_eq!(0, retval);
        assert_eq!(false, result);
    }

    #[test]
    fn test_string_length() {
        // Use a Rust "C string literal" to create a null-terminated string
        let s = c"Hello!";
        let len = unsafe { string_length(s.as_ptr()) };
        assert_eq!(6, len);

        let s = "Hello!";
        let s_c = std::ffi::CString::new(s).unwrap();
        let len = unsafe { string_length(s_c.as_ptr()) };
        assert_eq!(6, len);
    }

    #[test]
    fn test_string_length_and_delete() {
        // `into_raw` consumes the CString and returns a raw pointer
        let s = std::ffi::CString::new("Hello!").unwrap();
        let len = unsafe { string_length_and_delete(s.into_raw()) };
        assert_eq!(6, len);

        // Uncomment to show that this preserves Rust safety
        // println!("{:?}", s); // This will panic, as the CString has been deleted
    }

    #[test]
    fn test_return_hello() {
        let s = unsafe { return_hello() };

        // Convert the C string to a Rust string.
        // from_raw consumes the CString and returns the original String
        let s = unsafe { std::ffi::CString::from_raw(s) };
        let s = s.to_str().unwrap();
        assert_eq!("Hello", s);
    }

    #[test]
    fn test_call_me_maybe() {
        #[no_mangle]
        unsafe extern "C" fn maybe(n: i32) -> i32 {
            1
        }

        let n = unsafe { call_me(Some(maybe)) };
        assert_eq!(10, n); // Runs 10 times
    }

    #[test]
    fn test_constants() {
        assert_eq!(MY_CLIB_VERSION, 1);
        let c_str = unsafe { CStr::from_bytes_with_nul(MY_CLIB_VERSION_STRING)}.unwrap();
        let s = c_str.to_str().unwrap();
        assert_eq!(s, "1.0");
    }

    #[test]
    fn test_extract_from_the_void() {
        let my_struct = MyStruct { integer: 12, byte: 3 };
        let n = unsafe { extract_from_the_void(&my_struct as *const MyStruct as *mut c_void) };
        assert_eq!(n, 12);
    }

    #[test]
    fn testing_unions() {
        // This kinda makes sense
        let u = MyUnion { integer: 12 };
        assert_eq!(unsafe { u.integer }, 12);
        assert_eq!(unsafe { u.byte }, 12); // Technically undefined behavior, but it works

        // This is getting weird
        let u = MyUnion { integer: 512 };
        assert_eq!(unsafe { u.integer }, 512);
        assert_eq!(unsafe { u.byte }, 0); // You're just reading the first byte!

        // This is just wrong
        let u = MyUnion { byte: 1 };
        assert_eq!(unsafe { u.integer }, 1); // You're reading the first 4 bytes of a single byte!
    }
}