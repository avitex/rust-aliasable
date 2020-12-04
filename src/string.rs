//! Aliasable [`String`].

use alloc::string::String as UniqueString;
use core::ops::Deref;
use core::str;
use core::pin::Pin;

use crate::vec::Vec;

pub struct String(Vec<u8>);

impl String {
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    #[inline]
    pub fn into_unique(s: String) -> UniqueString {
        let unique_bytes = s.into_bytes().into();
        unsafe { UniqueString::from_utf8_unchecked(unique_bytes) }
    }

    pub fn into_unique_pinned(pin: Pin<String>) -> Pin<UniqueString> {
        unsafe {
            let aliasable = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(String::into_unique(aliasable))
        }
    }

    pub fn from_unique_pinned(pin: Pin<UniqueString>) -> Pin<String> {
        unsafe {
            let unique = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(String::from(unique))
        }
    }
}

impl From<UniqueString> for String {
    #[inline]
    fn from(s: UniqueString) -> Self {
        Self(s.into_bytes().into())
    }
}

impl From<String> for UniqueString {
    #[inline]
    fn from(s: String) -> Self {
        String::into_unique(s)
    }
}

impl Deref for String {
    type Target = str;

    #[inline]
    fn deref(self: &'_ Self) -> &'_ str {
        unsafe { str::from_utf8_unchecked(&*self.0) }
    }
}

impl AsRef<str> for String {
    fn as_ref(&self) -> &str {
        self.deref()
    }
}

#[cfg(feature = "traits")]
unsafe impl crate::StableDeref for String {}

#[cfg(feature = "traits")]
unsafe impl crate::AliasableDeref for String {}
