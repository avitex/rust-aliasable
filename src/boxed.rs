//! Aliasable `Box`.

use core::pin::Pin;
use core::{fmt, mem};
use core::{ops::Deref, ptr::NonNull};

pub use alloc::boxed::Box as UniqueBox;

pub struct AliasableBox<T: ?Sized>(NonNull<T>);

impl<T: ?Sized> AliasableBox<T> {
    /// Consumes the aliasable box and converts it back into a
    /// non-aliasable `Box`.
    #[inline]
    pub fn into_unique(mut ptr: AliasableBox<T>) -> UniqueBox<T> {
        // As we are consuming the `Box` structure we can safely assume any
        // aliasing has ended and convert the aliasable `Box` back to into an
        // unaliasable `UniqueBox`.
        let unique = unsafe { ptr.reclaim_as_unique_box() };
        // Forget the aliasable `Box` so the allocation behind the `UniqueBox`
        // is not deallocated.
        mem::forget(ptr);
        // Return the `UniqueBox`.
        unique
    }

    pub fn into_unique_pinned(pin: Pin<AliasableBox<T>>) -> Pin<UniqueBox<T>> {
        unsafe {
            let aliasable = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(AliasableBox::into_unique(aliasable))
        }
    }

    pub fn from_unique_pinned(pin: Pin<UniqueBox<T>>) -> Pin<AliasableBox<T>> {
        unsafe {
            let unique = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(AliasableBox::from(unique))
        }
    }

    #[inline]
    unsafe fn reclaim_as_unique_box(self: &'_ mut Self) -> UniqueBox<T> {
        UniqueBox::from_raw(self.0.as_ptr())
    }
}

impl<T: ?Sized> From<UniqueBox<T>> for AliasableBox<T> {
    #[inline]
    fn from(ptr: UniqueBox<T>) -> Self {
        let ptr = unsafe { NonNull::new_unchecked(UniqueBox::into_raw(ptr)) };
        Self(ptr)
    }
}

impl<T: ?Sized> Drop for AliasableBox<T> {
    fn drop(self: &'_ mut Self) {
        // As the `Box` structure is being dropped we can safely assume any
        // aliasing has ended and convert the aliasable `Box` back to into an
        // unaliasable `UniqueBox` to handle the deallocation.
        let _ = unsafe { self.reclaim_as_unique_box() };
    }
}

impl<T: ?Sized> Deref for AliasableBox<T> {
    type Target = T;

    #[inline]
    fn deref(self: &'_ Self) -> &'_ T {
        unsafe { self.0.as_ref() }
    }
}

impl<T: ?Sized> AsRef<T> for AliasableBox<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T: ?Sized> fmt::Debug for AliasableBox<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Box").field(&self.as_ref()).finish()
    }
}

#[cfg(feature = "traits")]
unsafe impl<T: ?Sized> crate::StableDeref for AliasableBox<T> {}

#[cfg(feature = "traits")]
unsafe impl<T: ?Sized> crate::AliasableDeref for AliasableBox<T> {}
