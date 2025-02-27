mod c_lib {
    // Ignore all warnings from the C code. Don't do this in production code!
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/clib.rs"));
}

mod rs {
    use std::ffi::c_char;

    pub fn double_byte(n: c_char) -> c_char { n.wrapping_mul(2) }
}

pub use rs::*;

#[cfg(test)]
mod tests {
    use std::os::raw::{c_char, c_void};
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
    fn test_factory_free() {
        let object = unsafe { factory() };
        // No null pointers for us!
        assert_ne!(object, std::ptr::null_mut());

        // We can call libc directly to free the memory
        // In this case, we've linked libc via the c_lib crate, sometimes
        // you have to import libc directly
        unsafe { free(object as *mut c_void) };
    }

    #[test]
    fn test_factory_box() {
        let object = unsafe { factory() };
        // No null pointers for us!
        assert_ne!(object, std::ptr::null_mut());

        let mut object = unsafe { Box::from_raw(object) };
        object.byte = 12;
        assert_eq!(object.byte, 12);

        // Let's check that we can still work with it
        let mut result = false;
        let retval = unsafe { is_byte_twelve(object.as_mut(), &mut result) };
        assert_eq!(0, retval);
        assert_eq!(true, result);
    }
}