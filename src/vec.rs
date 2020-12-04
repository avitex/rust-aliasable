//! Aliasable [`Vec`].

use core::{fmt, mem, slice};
use core::{ops::Deref, ptr::NonNull};

use alloc::vec::Vec as UniqueVec;

pub struct Vec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> Vec<T> {
    /// Consumes the aliasable [`Vec`] and converts it back into a
    /// non-aliasable [`UniqueVec`].
    #[inline]
    pub fn into_unique_vec(mut vec: Vec<T>) -> UniqueVec<T> {
        // As we are consuming the `Vec` structure we can safely assume any
        // aliasing has ended and convert the aliasable `Vec` back to into an
        // unaliasable `UniqueVec`.
        let unique = unsafe { vec.reclaim_as_unique_vec() };
        // Forget the aliasable `Vec` so the allocation behind the `UniqueVec`
        // is not deallocated.
        mem::forget(vec);
        // Return the `UniqueVec`.
        unique
    }

    #[inline]
    unsafe fn reclaim_as_unique_vec(self: &'_ mut Self) -> UniqueVec<T> {
        UniqueVec::from_raw_parts(self.ptr.as_mut(), self.len, self.cap)
    }
}

impl<T> From<UniqueVec<T>> for Vec<T> {
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

impl<T> From<Vec<T>> for UniqueVec<T> {
    #[inline]
    fn from(vec: Vec<T>) -> Self {
        Vec::into_unique_vec(vec)
    }
}

impl<T> Drop for Vec<T> {
    fn drop(self: &'_ mut Self) {
        // As the `Vec` structure is being dropped we can safely assume any
        // aliasing has ended and convert the aliasable `Vec` back to into an
        // unaliasable `UniqueVec` to handle the deallocation.
        let _ = unsafe { self.reclaim_as_unique_vec() };
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    #[inline]
    fn deref(self: &'_ Self) -> &'_ [T] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> AsRef<[T]> for Vec<T>  {
    fn as_ref(&self) -> &[T] {
        self.deref()
    }
}

impl<T> fmt::Debug for Vec<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Vec").field(&self.as_ref()).finish()
    }
}

#[cfg(feature = "traits")]
unsafe impl<T> crate::StableDeref for Vec<T> {}

#[cfg(feature = "traits")]
unsafe impl<T> crate::AliasableDeref for Vec<T> {}
