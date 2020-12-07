//! Aliasable `Vec`.

use core::pin::Pin;
use core::{fmt, mem, slice};
use core::{ops::Deref, ptr::NonNull};

pub use alloc::vec::Vec as UniqueVec;

/// A basic aliasable (non `core::ptr::Unique`) alternative to
/// [`alloc::vec::Vec`].
pub struct AliasableVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> AliasableVec<T> {
    /// Consumes the [`AliasableVec`] and converts it back into a
    /// non-aliasable [`UniqueVec`].
    #[inline]
    pub fn into_unique(mut vec: AliasableVec<T>) -> UniqueVec<T> {
        // SAFETY: As we are consuming the `Vec` structure we can safely assume
        // any aliasing has ended and convert the aliasable `Vec` back to into
        // an unaliasable `UniqueVec`.
        let unique = unsafe { vec.reclaim_as_unique_vec() };
        // Forget the aliasable `Vec` so the allocation behind the `UniqueVec`
        // is not deallocated.
        mem::forget(vec);
        // Return the `UniqueVec`.
        unique
    }

    /// Convert a pinned [`AliasableVec`] to a `core::ptr::Unique` backed pinned
    /// [`UniqueVec`].
    pub fn into_unique_pin(pin: Pin<AliasableVec<T>>) -> Pin<UniqueVec<T>> {
        // SAFETY: The pointer is not changed, just the container.
        unsafe {
            let aliasable = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(AliasableVec::into_unique(aliasable))
        }
    }

    /// Convert a pinned `core::ptr::Unique` backed [`UniqueVec`] to a
    /// pinned [`AliasableVec`].
    pub fn from_unique_pin(pin: Pin<UniqueVec<T>>) -> Pin<AliasableVec<T>> {
        unsafe {
            let unique = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(AliasableVec::from(unique))
        }
    }

    #[inline]
    unsafe fn reclaim_as_unique_vec(self: &'_ mut Self) -> UniqueVec<T> {
        UniqueVec::from_raw_parts(self.ptr.as_mut(), self.len, self.cap)
    }
}

impl<T> From<UniqueVec<T>> for AliasableVec<T> {
    #[inline]
    fn from(mut vec: UniqueVec<T>) -> Self {
        let ptr = vec.as_mut_ptr();
        let len = vec.len();
        let cap = vec.capacity();

        mem::forget(vec);

        let ptr = unsafe { NonNull::new_unchecked(ptr) };

        Self { ptr, len, cap }
    }
}

impl<T> From<AliasableVec<T>> for UniqueVec<T> {
    #[inline]
    fn from(vec: AliasableVec<T>) -> Self {
        AliasableVec::into_unique(vec)
    }
}

impl<T> Drop for AliasableVec<T> {
    fn drop(self: &'_ mut Self) {
        // As the `Vec` structure is being dropped we can safely assume any
        // aliasing has ended and convert the aliasable `Vec` back to into an
        // unaliasable `UniqueVec` to handle the deallocation.
        let _ = unsafe { self.reclaim_as_unique_vec() };
    }
}

impl<T> Deref for AliasableVec<T> {
    type Target = [T];

    #[inline]
    fn deref(self: &'_ Self) -> &'_ [T] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> AsRef<[T]> for AliasableVec<T> {
    fn as_ref(&self) -> &[T] {
        self.deref()
    }
}

impl<T> fmt::Debug for AliasableVec<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Vec").field(&self.as_ref()).finish()
    }
}

#[cfg(feature = "traits")]
unsafe impl<T> crate::StableDeref for AliasableVec<T> {}

#[cfg(feature = "traits")]
unsafe impl<T> crate::AliasableDeref for AliasableVec<T> {}
