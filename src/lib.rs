//! Basic aliasable (non `core::ptr::Unique`) types.
//!
//! # Why?
//!
//! Used for escaping `noalias` when multiple raw pointers may point to the same
//! data.

#![no_std]
#![forbid(
    clippy::pedantic,
    rust_2018_idioms,
    anonymous_parameters,
    unused_qualifications,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    warnings
)]

extern crate alloc;

pub mod boxed;
pub mod string;
pub mod vec;

#[cfg(feature = "traits")]
pub use aliasable_deref_trait::AliasableDeref;
#[cfg(feature = "traits")]
pub use stable_deref_trait::StableDeref;
