mod common;

use aliasable::prelude::{AliasableVec, UniqueVec};
use core::pin::Pin;

use self::common::{check_ordering, hash_of};

#[test]
fn test_new() {
    let aliasable = AliasableVec::from_unique(vec![10, 11]);
    assert_eq!(&*aliasable, &[10, 11]);
    let unique = AliasableVec::into_unique(aliasable);
    assert_eq!(&*unique, &[10, 11]);
}

#[test]
fn test_new_pin() {
    let aliasable = AliasableVec::from_unique_pin(Pin::new(vec![10]));
    assert_eq!(&*aliasable, &[10]);
    let unique = AliasableVec::into_unique_pin(aliasable);
    assert_eq!(&*unique, &[10]);
}

#[test]
fn test_refs() {
    let mut aliasable = AliasableVec::from_unique(vec![10]);
    let ptr: *const [u8] = &*aliasable;
    let as_mut_ptr: *const [u8] = aliasable.as_mut();
    let as_ref_ptr: *const [u8] = aliasable.as_ref();
    assert_eq!(ptr, as_mut_ptr);
    assert_eq!(ptr, as_ref_ptr);
}

#[test]
fn test_debug() {
    let aliasable = AliasableVec::from_unique(vec![10]);
    assert_eq!(format!("{:?}", aliasable), "[10]");
}

#[test]
fn test_default() {
    assert_eq!(
        <AliasableVec<&i32>>::default(),
        <AliasableVec<&i32>>::from_unique(UniqueVec::new())
    );
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_clone() {
    let mut v = AliasableVec::from_unique(vec![1, 2, 3]);
    assert_eq!(&*v.clone(), [1, 2, 3]);

    v.clone_from(&AliasableVec::from_unique(vec![4, 5, 6, 7, 8]));
    assert_eq!(&*v, [4, 5, 6, 7, 8]);

    v.clone_from(&AliasableVec::from_unique(vec![9]));
    assert_eq!(&*v, [9]);

    v.clone_from(&AliasableVec::default());
    assert_eq!(&*v, []);
}

#[test]
fn test_cmp() {
    check_ordering(
        AliasableVec::from_unique(vec![1, 2, 3]),
        AliasableVec::from_unique(vec![1, 2, 4]),
    );

    let l = <AliasableVec<&str>>::from_unique(vec!["x", "y"]);
    let r = <AliasableVec<String>>::from_unique(vec!["x".into(), "y".into()]);
    assert_eq!(l, r);
}

#[test]
fn test_hash() {
    assert_eq!(
        hash_of(AliasableVec::from_unique(vec![1, 2, 3])),
        hash_of([1, 2, 3])
    );
}
