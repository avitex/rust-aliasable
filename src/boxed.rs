//! Aliasable `Box`.

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::ptr::NonNull;

pub use alloc::boxed::Box as UniqueBox;

/// Basic aliasable (non `core::ptr::Unique`) alternative to
/// [`alloc::boxed::Box`].
pub struct AliasableBox<T: ?Sized>(NonNull<T>);

impl<T: ?Sized> AliasableBox<T> {
    /// Construct an `AliasableBox` from a [`UniqueBox`].
    pub fn from_unique(unique: UniqueBox<T>) -> Self {
        // Leak the refence to the allocation from the unique box.
        let leaked_ref = UniqueBox::leak(unique);
        // Return the aliasable box.
        Self(NonNull::from(leaked_ref))
    }

    /// Consumes `self` and converts it into a non-aliasable [`UniqueBox`].
    #[inline]
    pub fn into_unique(aliasable: AliasableBox<T>) -> UniqueBox<T> {
        // Ensure we don't drop `self` as we are transferring the allocation and
        // we don't want a use after free.
        let mut aliasable = ManuallyDrop::new(aliasable);
        // SAFETY: As we are consuming the aliasable box we can safely assume
        // any aliasing has ended and convert the aliasable box back to into an
        // unique box.
        unsafe { aliasable.reclaim_as_unique_box() }
    }

    /// Convert a pinned [`AliasableBox`] to a `core::ptr::Unique` backed pinned
    /// [`UniqueBox`].
    pub fn into_unique_pin(pin: Pin<AliasableBox<T>>) -> Pin<UniqueBox<T>> {
        // SAFETY: The pointer is not changed, just the container.
        unsafe {
            let aliasable = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(AliasableBox::into_unique(aliasable))
        }
    }

    /// Convert a pinned `core::ptr::Unique` backed [`UniqueBox`] to a
    /// pinned [`AliasableBox`].
    pub fn from_unique_pin(pin: Pin<UniqueBox<T>>) -> Pin<AliasableBox<T>> {
        // SAFETY: The pointer is not changed, just the container.
        unsafe {
            let unique = Pin::into_inner_unchecked(pin);
            Pin::new_unchecked(AliasableBox::from(unique))
        }
    }

    #[inline]
    unsafe fn reclaim_as_unique_box(&mut self) -> UniqueBox<T> {
        UniqueBox::from_raw(self.0.as_ptr())
    }
}

impl<T: ?Sized> From<UniqueBox<T>> for AliasableBox<T> {
    fn from(unique: UniqueBox<T>) -> Self {
        Self::from_unique(unique)
    }
}

impl<T: ?Sized> Drop for AliasableBox<T> {
    fn drop(&mut self) {
        // SAFETY: As `self` is being dropped we can safely assume any aliasing
        // has ended and convert the aliasable box back to into an unique box to
        // handle the deallocation.
        let _box = unsafe { self.reclaim_as_unique_box() };
    }
}

impl<T: ?Sized> Deref for AliasableBox<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        // SAFETY: We own the data, so we can return a reference to it.
        unsafe { self.0.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for AliasableBox<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: We own the data, so we can return a reference to it.
        unsafe { self.0.as_mut() }
    }
}

impl<T: ?Sized> AsRef<T> for AliasableBox<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &*self
    }
}

impl<T: ?Sized> AsMut<T> for AliasableBox<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut *self
    }
}

impl<T: ?Sized> fmt::Debug for AliasableBox<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_ref(), f)
    }
}

unsafe impl<T: ?Sized> Send for AliasableBox<T> where T: Send {}
unsafe impl<T: ?Sized> Sync for AliasableBox<T> where T: Sync {}

impl<T: Default> Default for AliasableBox<T> {
    #[inline]
    fn default() -> Self {
        Self::from_unique(UniqueBox::default())
    }
}

impl<T: Clone> Clone for AliasableBox<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::from_unique(UniqueBox::new((**self).clone()))
    }
    #[inline]
    fn clone_from(&mut self, source: &Self) {
        (**self).clone_from(&**source);
    }
}

impl<T: PartialEq + ?Sized> PartialEq for AliasableBox<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T: Eq + ?Sized> Eq for AliasableBox<T> {}

impl<T: PartialOrd + ?Sized> PartialOrd for AliasableBox<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
    #[inline]
    fn lt(&self, other: &Self) -> bool {
        **self < **other
    }
    #[inline]
    fn le(&self, other: &Self) -> bool {
        **self <= **other
    }
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        **self > **other
    }
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        **self >= **other
    }
}

impl<T: Ord + ?Sized> Ord for AliasableBox<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: Hash + ?Sized> Hash for AliasableBox<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

#[cfg(feature = "stable_deref_trait")]
unsafe impl<T: ?Sized> crate::StableDeref for AliasableBox<T> {}

#[cfg(feature = "aliasable_deref_trait")]
unsafe impl<T: ?Sized> crate::AliasableDeref for AliasableBox<T> {}

#[cfg(feature = "unsize")]
unsafe impl<T, U: ?Sized> unsize::CoerciblePtr<U> for AliasableBox<T> {
    type Pointee = T;
    type Output = AliasableBox<U>;

    fn as_sized_ptr(&mut self) -> *mut T {
        self.0.as_ptr()
    }

    unsafe fn replace_ptr(self, new: *mut U) -> AliasableBox<U> {
        // Ensure we don't drop `self` as we are transferring the allocation and
        // we don't want a use after free.
        let this = ManuallyDrop::new(self);
        // Replace the inner pointer type.
        let ptr = this.0.replace_ptr(new);
        // Return the aliasable box with the new pointer.
        AliasableBox(ptr)
    }
}
