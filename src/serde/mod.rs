#![allow(clippy::integer_arithmetic)]
mod de;
mod error;
mod ser;

pub use de::{from_bytes, from_msg};
pub use ser::{to_bytes, to_msg};
pub use error::{Error, Result};
