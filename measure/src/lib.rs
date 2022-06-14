#![allow(clippy::integer_arithmetic)]
pub mod macros;
pub mod measure;

#[cfg(feature = "tsc")]
#[cfg(target_arch = "x86_64")]
pub use crate::measure::MeasureTsc as Measure;

#[cfg(not(feature = "tsc"))]
pub use crate::measure::Measure;
