//! Aliasable `Vec`.

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::ptr::NonNull;
use core::{fmt, mem, slice};

pub use alloc::vec::Vec as UniqueVec;

/// Basic aliasable (non `core::ptr::Unique`) alternative to
/// [`alloc::vec::Vec`].
pub struct AliasableVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> AliasableVec<T> {
    /// Returns the number of elements in the vector, also referred to as its
    /// ‘length’.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the number of elements the vector can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Returns a raw pointer to the vector’s buffer.
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    /// Returns an unsafe mutable pointer to the vector’s buffer.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Construct an `AliasableVec` from a [`UniqueVec`].
    pub fn from_unique(unique: UniqueVec<T>) -> Self {
        // Ensure we don't drop `self` as we are transferring the allocation and
        // we don't want a use after free.
        let mut unique = ManuallyDrop::new(unique);

        // Get the raw parts of the vector.
        let ptr = unique.as_mut_ptr();
        let len = unique.len();
        let cap = unique.capacity();

        // SAFETY: The pointer returned by a vec is never null.
        let ptr = unsafe { NonNull::new_unchecked(ptr) };

        // Return the aliasable vec.
        Self { ptr, len, cap }
    }

    /// Consumes the [`AliasableVec`] and converts it back into a
    /// non-aliasable [`UniqueVec`].
    #[inline]
    pub fn into_unique(aliasable: AliasableVec<T>) -> UniqueVec<T> {
        // Ensure we don't drop `self` as we are transferring the allocation and
        // we don't want a use after free.
        let mut aliasable = ManuallyDrop::new(aliasable);
        // SAFETY: As we are consuming the aliasable vec we can safely assume
        // any aliasing has ended and convert the aliasable vec back to into an
        // unique vec.
        unsafe { aliasable.reclaim_as_unique_vec() }
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
    unsafe fn reclaim_as_unique_vec(&mut self) -> UniqueVec<T> {
        UniqueVec::from_raw_parts(self.ptr.as_ptr(), self.len, self.cap)
    }
}

impl<T> From<UniqueVec<T>> for AliasableVec<T> {
    #[inline]
    fn from(unique: UniqueVec<T>) -> Self {
        Self::from_unique(unique)
    }
}

impl<T> From<AliasableVec<T>> for UniqueVec<T> {
    #[inline]
    fn from(aliasable: AliasableVec<T>) -> Self {
        AliasableVec::into_unique(aliasable)
    }
}

impl<T> Drop for AliasableVec<T> {
    fn drop(&mut self) {
        // SAFETY: As `self` is being dropped we can safely assume any aliasing
        // has ended and convert the aliasable vec back to into an unique vec to
        // handle the deallocation.
        let _vec = unsafe { self.reclaim_as_unique_vec() };
    }
}

impl<T> Deref for AliasableVec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        // SAFETY: We own the data, so we can return a reference to it.
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for AliasableVec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        // SAFETY: We own the data, so we can return a reference to it.
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> AsRef<[T]> for AliasableVec<T> {
    fn as_ref(&self) -> &[T] {
        &*self
    }
}

impl<T> AsMut<[T]> for AliasableVec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut *self
    }
}

impl<T> fmt::Debug for AliasableVec<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_ref(), f)
    }
}

unsafe impl<T> Send for AliasableVec<T> where T: Send {}
unsafe impl<T> Sync for AliasableVec<T> where T: Sync {}

impl<T> Default for AliasableVec<T> {
    #[inline]
    fn default() -> Self {
        Self::from_unique(UniqueVec::new())
    }
}

impl<T: Clone> Clone for AliasableVec<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::from_unique((**self).to_vec())
    }
    #[inline]
    fn clone_from(&mut self, source: &Self) {
        struct Guard<'a, T>(&'a mut AliasableVec<T>, UniqueVec<T>);
        impl<T> Drop for Guard<'_, T> {
            fn drop(&mut self) {
                *self.0 = AliasableVec::from_unique(mem::take(&mut self.1));
            }
        }

        let taken = Self::into_unique(mem::take(self));
        let mut guard = Guard(self, taken);

        guard.1.truncate(source.len);

        let (init, tail) = source.split_at(guard.1.len());

        guard.1.clone_from_slice(init);
        guard.1.extend_from_slice(tail);
    }
}

impl<T: PartialEq<U>, U> PartialEq<AliasableVec<U>> for AliasableVec<T> {
    #[inline]
    fn eq(&self, other: &AliasableVec<U>) -> bool {
        **self == **other
    }
}

impl<T: Eq> Eq for AliasableVec<T> {}

impl<T: PartialOrd> PartialOrd for AliasableVec<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T: Ord> Ord for AliasableVec<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: Hash> Hash for AliasableVec<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

#[cfg(feature = "stable_deref_trait")]
unsafe impl<T> crate::StableDeref for AliasableVec<T> {}

#[cfg(feature = "aliasable_deref_trait")]
unsafe impl<T> crate::AliasableDeref for AliasableVec<T> {}
