mod common;

use aliasable::prelude::{AliasableString, AliasableVec, UniqueString};
use core::pin::Pin;

use self::common::{check_ordering, hash_of};

#[test]
fn test_new() {
    let aliasable = AliasableString::from_unique(UniqueString::from("hello"));
    assert_eq!(&*aliasable, "hello");
    let unique = AliasableString::into_unique(aliasable);
    assert_eq!(&*unique, "hello");
}

#[test]
fn test_new_pin() {
    let aliasable = AliasableString::from_unique_pin(Pin::new(UniqueString::from("hello")));
    assert_eq!(&*aliasable, "hello");
    let unique = AliasableString::into_unique_pin(aliasable);
    assert_eq!(&*unique, "hello");
}

#[test]
fn test_refs() {
    let mut aliasable = AliasableString::from_unique(UniqueString::from("hello"));
    let ptr: *const str = &*aliasable;
    let as_mut_ptr: *const str = aliasable.as_mut();
    let as_ref_ptr: *const str = aliasable.as_ref();
    assert_eq!(ptr, as_mut_ptr);
    assert_eq!(ptr, as_ref_ptr);
}

#[test]
fn test_debug() {
    let aliasable = AliasableString::from_unique(UniqueString::from("hello"));
    assert_eq!(format!("{:?}", aliasable), "\"hello\"");
}

#[test]
fn test_into_bytes() {
    let aliasable = AliasableString::from_unique(UniqueString::from("hello"));
    assert_eq!(
        AliasableVec::into_unique(aliasable.into_bytes()),
        vec![b'h', b'e', b'l', b'l', b'o']
    );
}

#[test]
fn test_default() {
    assert_eq!(
        AliasableString::default(),
        AliasableString::from_unique(UniqueString::new())
    );
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_clone() {
    let mut s = AliasableString::from_unique("hello".into());
    assert_eq!(&*s.clone(), "hello");
    s.clone_from(&AliasableString::from_unique("world".into()));
    assert_eq!(&*s, "world");
}

#[test]
fn test_cmp() {
    check_ordering(
        AliasableString::from_unique("abcdef".into()),
        AliasableString::from_unique("abdef".into()),
    );
}

#[test]
fn test_hash() {
    assert_eq!(
        hash_of(AliasableString::from_unique("some data".into())),
        hash_of("some data")
    );
}
