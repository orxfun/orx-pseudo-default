//! # orx-pseudo-default
//!
//! [![orx-pseudo-default crate](https://img.shields.io/crates/v/orx-pseudo-default.svg)](https://crates.io/crates/orx-pseudo-default)
//! [![orx-pseudo-default documentation](https://docs.rs/orx-pseudo-default/badge.svg)](https://docs.rs/orx-pseudo-default)
//!
//! `PseudoDefault` is a simple trait for creating a default instance of a type, which is not **necessarily useful**.
//!
//! A trait for creating a default instance of a type, which is not **necessarily useful**.
//!
//! The difference of `PseudoDefault` from `Default` is the relaxed expectation of the created instance to be useful.
//!
//! The main use case of the trait is when we need to create a cheap instance of a type without any arguments to throw away.
//! This sounds exactly the same as the `Default`s definition.
//! However, the instance created by `pseudo_default` does not need to be a decent one, it is totally fine if it is useless.
//!
//! We would most possibly need such an instance when we need to swap it with another instance or fill a gap in a collection.
//! `PseudoDefault` allows us to achieve this without requiring to use unsafe code and leaving the memory location uninitialized.
//!
//! # Example
//!
//! Consider the following fictional type `Share` which divides a whole into pieces.
//! Without providing the `number_of_shares`, this type is meaningless; i.e., we cannot justify implementing `Default`.
//! However, if we need to be able to create dummy Share's, we can simply use `pseudo_default`.
//! The created share will be useless; however, a valid and safe `Share` instance.
//!
//! ```rust
//! use orx_pseudo_default::PseudoDefault;
//!
//! struct Share {
//!     number_of_shares: std::num::NonZeroUsize,
//! }
//!
//! impl Share {
//!     fn share_size(&self, whole_amount: usize) -> usize {
//!         whole_amount / self.number_of_shares
//!     }
//! }
//!
//! impl PseudoDefault for Share {
//!     fn pseudo_default() -> Self {
//!         Self {
//!             number_of_shares: std::num::NonZeroUsize::new(1).unwrap(),
//!         }
//!     }
//! }
//! ```
//!
//! ## License
//!
//! This library is licensed under MIT license. See LICENSE for details.

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

mod pseudo_default;

pub use pseudo_default::PseudoDefault;
