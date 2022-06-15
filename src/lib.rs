//#[macro_use(lazy_static)]
//extern crate lazy_static;

pub mod measure;

//#[cfg(target_endian = "little")]

#[cfg(test)]
mod tests {
    #[test]
    fn u256_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        #[cfg(not(target_endian = "little"))]
        assert!(false, "Not little endian platform");
    }
}
