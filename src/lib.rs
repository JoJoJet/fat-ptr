use std::marker::PhantomData;

/// A fat pointer to zero or more values of type `T`,
/// which stores arbitrary metadata of type `M`.
/// M must be trivially convertable to and from a `usize`.
#[repr(transparent)]
pub struct Fat<T, M: Meta>(PhantomData<T>, PhantomData<*mut M>, [()]);

/// A value that can act as the metadata in a fat pointer.
pub trait Meta: Sized {
    /// Converts this value into a pointer-width string of bytes stored as a `usize`.
    /// If the type is the same size as `usize`, this can just be a transmute.
    fn into_bytes(self) -> usize;

    /// Gets metadata from bytes stored in as a `usize`.
    /// # Safety
    /// The argument for this fn must have come from calling `Self::into_bytes()`.
    /// Thus, it should be a valid bit pattern for `Self`.
    unsafe fn from_bytes(_: usize) -> Self;
}

impl<T, M: Meta> Fat<T, M> {
    pub fn ptr(&self) -> *const T {
        self.2.as_ptr() as *const T
    }
    pub fn mut_ptr(&mut self) -> *mut T {
        self.2.as_mut_ptr() as *mut T
    }
    pub fn meta(&self) -> M {
        // `Fat` can only be created by the `from_*` fns, which all use
        // `Meta::into_bytes` on the metadata.
        // Thus, it is sound to call `Meta::from_bytes`
        unsafe { M::from_bytes(self.2.len()) }
    }

    pub fn from_slice(data: &[T], meta: M) -> &Self {
        let ptr = data.as_ptr() as *const ();
        // SAFETY: Creating this slice is sound, as `slice::from_raw_parts` requires
        // `ptr` to point to `dim` fully-initialized and aligned values of ().
        // () is a ZST, so it is fully initialized and aligned no matter what.
        let fat = unsafe { std::slice::from_raw_parts(ptr, meta.into_bytes()) };

        // SAFETY: `Fat` is repr(transparent), so it's sound to transmute
        // from &[()] -> &Fat<T, M>.
        unsafe { std::mem::transmute(fat) }
    }
    pub fn from_slice_mut(data: &mut [T], meta: M) -> &mut Self {
        let ptr = data.as_mut_ptr() as *mut ();
        // SAFETY: Creating this slice is sound, as `slice::from_raw_parts_mut` requires
        // `ptr` to point to `dim` fully-initialized and aligned values of ().
        // () is a ZST, so it is fully initialized and aligned no matter what.
        let fat = unsafe { std::slice::from_raw_parts_mut(ptr, meta.into_bytes()) };

        // SAFETY: `Fat` is repr(transparent), so it's sound to transmute
        // from &mut [()] -> &mut Fat<T, M>.
        unsafe { std::mem::transmute(fat) }
    }
}
