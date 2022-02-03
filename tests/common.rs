extern crate alloc;

use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt::Debug;
use core::hash::{Hash, Hasher};

#[allow(clippy::eq_op)]
pub fn check_ordering<T: PartialEq + Eq + PartialOrd + Ord + Debug>(a: T, b: T) {
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

pub fn hash_of(value: impl Hash) -> Vec<u8> {
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
