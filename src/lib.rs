#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]
#![no_std]

#[cfg(any(test, feature = "std"))]
extern crate std;

extern crate alloc;

mod implementations;
mod pseudo_default;

pub use pseudo_default::PseudoDefault;

#[cfg(feature = "derive")]
pub use orx_pseudo_default_derive::PseudoDefault;
