//! # Surface API
//!
//! Frozen `repr(C)` ABI definitions and safe zero-cost convenience layers for
//! the Oxide-Tech Autoresearch training surface.

pub mod abi;
pub mod prelude;
pub mod safe;

pub use abi::*;
