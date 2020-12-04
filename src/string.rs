//! Aliasable `String`.

use core::ops::Deref;
use core::pin::Pin;
use core::str;

use crate::vec::AliasableVec;

pub use alloc::string::String as UniqueString;

pub struct AliasableString(AliasableVec<u8>);

impl AliasableString {
    pub fn into_bytes(self) -> AliasableVec<u8> {
        self.0
    }

    #[inline]
    pub fn into_unique(s: AliasableString) -> UniqueString {
        let unique_bytes = s.into_bytes().into();
        unsafe { UniqueString::from_utf8_unchecked(unique_bytes) }
    }

    pub fn into_unique_pinned(pin: Pin<AliasableString>) -> Pin<UniqueString> {
        unsafe {
            let aliasable = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(AliasableString::into_unique(aliasable))
        }
    }

    pub fn from_unique_pinned(pin: Pin<UniqueString>) -> Pin<AliasableString> {
        unsafe {
            let unique = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(AliasableString::from(unique))
        }
    }
}

impl From<UniqueString> for AliasableString {
    #[inline]
    fn from(s: UniqueString) -> Self {
        Self(s.into_bytes().into())
    }
}

impl From<AliasableString> for UniqueString {
    #[inline]
    fn from(s: AliasableString) -> Self {
        AliasableString::into_unique(s)
    }
}

impl Deref for AliasableString {
    type Target = str;

    #[inline]
    fn deref(self: &'_ Self) -> &'_ str {
        unsafe { str::from_utf8_unchecked(&*self.0) }
    }
}

impl AsRef<str> for AliasableString {
    fn as_ref(&self) -> &str {
        self.deref()
    }
}

#[cfg(feature = "traits")]
unsafe impl crate::StableDeref for AliasableString {}

#[cfg(feature = "traits")]
unsafe impl crate::AliasableDeref for AliasableString {}
