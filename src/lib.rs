//! Basic aliasable (non `core::ptr::Unique`) types.
//!
//! # Why?
//!
//! Used for escaping `noalias` when multiple raw pointers may point to the same
//! data.

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(
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
#![allow(
    clippy::needless_pass_by_value,
    clippy::wrong_self_convention,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

mod mut_ref;

#[cfg(feature = "alloc")]
pub mod boxed;
#[cfg(feature = "alloc")]
pub mod string;
#[cfg(feature = "alloc")]
pub mod vec;

pub use crate::mut_ref::AliasableMut;

/// Export of all types enabled.
pub mod prelude {
    #[cfg(feature = "alloc")]
    pub use crate::boxed::*;
    #[cfg(feature = "alloc")]
    pub use crate::string::*;
    #[cfg(feature = "alloc")]
    pub use crate::vec::*;

    pub use crate::mut_ref::*;
}

#[cfg(feature = "aliasable_deref_trait")]
pub use aliasable_deref_trait::AliasableDeref;
#[cfg(feature = "stable_deref_trait")]
pub use stable_deref_trait::StableDeref;

#[cfg(test)]
mod test_utils {
    use alloc::vec::Vec;
    use core::cmp::Ordering;
    use core::fmt::Debug;
    use core::hash::{Hash, Hasher};

    #[allow(clippy::eq_op)]
    pub(crate) fn check_ordering<T: PartialEq + Eq + PartialOrd + Ord + Debug>(a: T, b: T) {
        assert_eq!(a, a);
        assert_eq!(b, b);
        assert_ne!(a, b);
        assert_ne!(b, a);

        assert_eq!(a.cmp(&a), Ordering::Equal);
        assert_eq!(b.cmp(&b), Ordering::Equal);
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);

        assert_eq!(a.partial_cmp(&a).unwrap(), Ordering::Equal);
        assert_eq!(b.partial_cmp(&b).unwrap(), Ordering::Equal);
        assert_eq!(a.partial_cmp(&b).unwrap(), Ordering::Less);
        assert_eq!(b.partial_cmp(&a).unwrap(), Ordering::Greater);

        assert!(!(a < a));
        assert!(!(b < b));
        assert!(a < b);
        assert!(!(b < a));

        assert!(!(a > a));
        assert!(!(b > b));
        assert!(!(a > b));
        assert!(b > a);

        assert!(a <= a);
        assert!(b <= b);
        assert!(a <= b);
        assert!(!(b <= a));

        assert!(a >= a);
        assert!(b >= b);
        assert!(!(a >= b));
        assert!(b >= a);
    }

    pub(crate) fn hash_of(value: impl Hash) -> Vec<u8> {
        #[derive(Default, PartialEq)]
        struct DummyHasher(Vec<u8>);
        impl Hasher for DummyHasher {
            fn finish(&self) -> u64 {
                0
            }
            fn write(&mut self, bytes: &[u8]) {
                self.0.extend_from_slice(bytes);
            }
        }

        let mut hasher = DummyHasher::default();
        value.hash(&mut hasher);
        hasher.0
    }
}
