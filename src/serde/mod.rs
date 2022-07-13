//! serde - Serialize/Deserialize for Little End
//!
//! It aims to sensibly handle x86_64/aarch64 CPU
//! It is zero-allocation and pretty fast. It will process
//! several million messages per second on a decent CPU.
//!
//!
#![allow(clippy::integer_arithmetic)]
mod de;
mod error;
mod ser;

pub use de::{from_bytes, from_msg};
pub use error::{Error, Result};
pub use ser::{to_bytes, to_msg};
