#![allow(clippy::integer_arithmetic)]
mod de;
mod error;
mod ser;

pub use de::from_msg;
pub use ser::to_msg;
