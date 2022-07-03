//! pitch - a parser for the SHFE Ponorama ITCH protocol 1.0
//!
//! It aims to sensibly handle the whole protocol.
//! It is zero-allocation and pretty fast. It will process
//! several million messages per second on a decent CPU.
//!
//!
//! The protocol specification can be found on the [SHFE website](http://www.shfe.comcn/PITCHSpecification.pdf)

mod enums;
mod pitch;
mod proto;

pub use enums::*;
