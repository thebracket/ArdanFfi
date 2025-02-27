mod c_lib {
    include!(concat!(env!("OUT_DIR"), "/clib.rs"));
}

#[cfg(test)]
mod tests {
    use super::c_lib::*;

    #[test]
    fn test_double() {
        unsafe {
            assert_eq!(double_byte(2), 4);
        }
    }
}