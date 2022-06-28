#[macro_use(lazy_static)]
extern crate lazy_static;

mod datetime;
mod julian;
pub mod measure;
mod msg;
mod price_type;
mod serde;
mod timestamp;
mod unix_time;

#[cfg(target_arch = "x86_64")]
pub mod x86;

pub mod u64;
pub use datetime::DateTime;
pub use julian::Julian;
pub use msg::ClMessaage;
pub use price_type::PriceType;
pub use timestamp::{nsleep, SysClock, TimeVal};
pub use unix_time::{Local, UnixTime};

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
