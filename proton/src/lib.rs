//! Proton: A library for converting inches to meters.
//!
//! This library provides core conversion functionality and error handling
//! for converting between inches and meters.

mod conversion;

pub use conversion::{convert_inches_to_meters, InchesToMetersError};
