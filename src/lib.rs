//#[macro_use(lazy_static)]
//extern crate lazy_static;

mod julian;
pub mod measure;
mod price_type;

#[cfg(target_arch = "x86_64")]
pub mod x86;

pub use julian::Julian;
pub use price_type::PriceType;

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
