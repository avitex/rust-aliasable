mod common;

use aliasable::prelude::{AliasableMut, UniqueBox};
use core::pin::Pin;

use self::common::{check_ordering, hash_of};

#[test]
fn test_new() {
    let mut data = UniqueBox::new(10);
    let aliasable = AliasableMut::from_unique(&mut data);
    assert_eq!(**aliasable, 10);
    let unique = AliasableMut::into_unique(aliasable);
    assert_eq!(**unique, 10);
}

#[test]
fn test_new_pin() {
    let mut data = UniqueBox::new(10);
    let data = unsafe { Pin::new_unchecked(&mut data) };
    let aliasable = AliasableMut::from_unique_pin(data);
    assert_eq!(**aliasable, 10);
    let unique = AliasableMut::into_unique_pin(aliasable);
    assert_eq!(**unique, 10);
}

#[test]
fn test_refs() {
    let mut data = UniqueBox::new(10);
    let mut aliasable = AliasableMut::from_unique(&mut data);
    let ptr: *const UniqueBox<u8> = &mut *aliasable;
    let as_mut_ptr: *const UniqueBox<u8> = aliasable.as_mut();
    let as_ref_ptr: *const UniqueBox<u8> = aliasable.as_ref();
    assert_eq!(ptr, as_mut_ptr);
    assert_eq!(ptr, as_ref_ptr);
}

#[test]
fn test_debug() {
    let mut data = 10;
    let aliasable = AliasableMut::from_unique(&mut data);
    assert_eq!(format!("{:?}", aliasable), "10");
}

#[test]
fn test_cmp() {
    check_ordering(
        AliasableMut::from_unique(&mut 5),
        AliasableMut::from_unique(&mut 7),
    );
}

#[test]
fn test_hash() {
    assert_eq!(hash_of(AliasableMut::from_unique(&mut 389)), hash_of(389));
}
