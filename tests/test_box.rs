mod common;

use aliasable::prelude::{AliasableBox, UniqueBox};

use self::common::{check_ordering, hash_of};

#[test]
fn test_new() {
    let aliasable = AliasableBox::from_unique(UniqueBox::new(10));
    assert_eq!(*aliasable, 10);
    let unique = AliasableBox::into_unique(aliasable);
    assert_eq!(*unique, 10);
}

#[test]
fn test_new_pin() {
    let aliasable = AliasableBox::from_unique_pin(UniqueBox::pin(10));
    assert_eq!(*aliasable, 10);
    let unique = AliasableBox::into_unique_pin(aliasable);
    assert_eq!(*unique, 10);
}

#[test]
fn test_refs() {
    let mut aliasable = AliasableBox::from_unique(UniqueBox::new(10));
    let ptr: *const u8 = &*aliasable;
    let as_mut_ptr: *const u8 = aliasable.as_mut();
    let as_ref_ptr: *const u8 = aliasable.as_ref();
    assert_eq!(ptr, as_mut_ptr);
    assert_eq!(ptr, as_ref_ptr);
}

#[test]
fn test_debug() {
    let aliasable = AliasableBox::from_unique(UniqueBox::new(10));
    assert_eq!(format!("{:?}", aliasable), "10");
}

#[test]
fn test_default() {
    assert_eq!(*<AliasableBox<i32>>::default(), 0);
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_clone() {
    let mut boxed = AliasableBox::from_unique(UniqueBox::new(10));
    assert_eq!(*boxed.clone(), 10);
    boxed.clone_from(&AliasableBox::from_unique(UniqueBox::new(20)));
    assert_eq!(*boxed, 20);
}

#[test]
fn test_cmp() {
    check_ordering(
        AliasableBox::from_unique(UniqueBox::new(5)),
        AliasableBox::from_unique(UniqueBox::new(7)),
    );
}

#[test]
fn test_hash() {
    let b = UniqueBox::new(5);
    assert_eq!(hash_of(AliasableBox::from_unique(b.clone())), hash_of(b));
}

#[cfg(feature = "unsize")]
#[test]
fn test_unsize() {
    use unsize::{CoerceUnsize, Coercion};
    let aliasable = AliasableBox::from_unique(UniqueBox::new([0u8; 2]));
    let unsized_box: AliasableBox<[u8]> = aliasable.unsize(Coercion::to_slice());
    assert_eq!(*unsized_box, [0, 0]);
}
