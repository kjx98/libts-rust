#[macro_use(lazy_static)]
extern crate lazy_static;

mod datetime;
mod julian;
pub mod measure;
mod msg;
pub mod pitch;
mod price_type;
mod serde;
mod timestamp;
pub mod u64;
mod unix_time;

#[cfg(target_arch = "x86_64")]
pub mod x86;

pub use crate::serde::{from_bytes, from_msg, to_bytes, to_msg};
pub use datetime::DateTime;
pub use julian::Julian;
pub use msg::ClMessage;
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
