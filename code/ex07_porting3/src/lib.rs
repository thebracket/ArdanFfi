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
    use std::os::raw::c_char;

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
}