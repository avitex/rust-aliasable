//! Aliasable `String`.

use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::{fmt, str};

use crate::vec::AliasableVec;

pub use alloc::string::String as UniqueString;

/// A basic aliasable (non `core::ptr::Unique`) alternative to
/// [`alloc::string::String`].
pub struct AliasableString(AliasableVec<u8>);

impl AliasableString {
    /// Consumes `self` into an [`AliasableVec`] of UTF-8 bytes.
    pub fn into_bytes(self) -> AliasableVec<u8> {
        self.0
    }

    /// Consumes `self` and converts it into a non-aliasable [`UniqueString`].
    #[inline]
    pub fn into_unique(s: AliasableString) -> UniqueString {
        let unique_bytes = s.into_bytes().into();
        unsafe { UniqueString::from_utf8_unchecked(unique_bytes) }
    }

    /// Convert a pinned [`AliasableString`] to a `core::ptr::Unique` backed pinned
    /// [`UniqueString`].
    pub fn into_unique_pin(pin: Pin<AliasableString>) -> Pin<UniqueString> {
        // SAFETY: The pointer is not changed, just the container.
        unsafe {
            let aliasable = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(AliasableString::into_unique(aliasable))
        }
    }

    /// Convert a pinned `core::ptr::Unique` backed [`UniqueString`] to a
    /// pinned [`AliasableString`].
    pub fn from_unique_pin(pin: Pin<UniqueString>) -> Pin<AliasableString> {
        // SAFETY: The pointer is not changed, just the container.
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
    fn deref(self: &Self) -> &str {
        // SAFETY: `AliasableString` will only ever contain UTF-8.
        unsafe { str::from_utf8_unchecked(&*self.0) }
    }
}

impl DerefMut for AliasableString {
    #[inline]
    fn deref_mut(self: &mut Self) -> &mut str {
        // SAFETY: `AliasableString` will only ever contain UTF-8.
        unsafe { str::from_utf8_unchecked_mut(&mut *self.0) }
    }
}

impl AsRef<str> for AliasableString {
    #[inline]
    fn as_ref(&self) -> &str {
        self.deref()
    }
}

impl AsMut<str> for AliasableString {
    fn as_mut(&mut self) -> &mut str {
        self.deref_mut()
    }
}

impl fmt::Debug for AliasableString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_ref(), f)
    }
}

#[cfg(feature = "traits")]
unsafe impl crate::StableDeref for AliasableString {}

#[cfg(feature = "traits")]
unsafe impl crate::AliasableDeref for AliasableString {}
