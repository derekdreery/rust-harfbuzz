use crate::{
    blob::{Blob, Borrowed, Owned},
    sys,
};
use std::{marker::PhantomData, os::raw::c_uint, sync::Arc};

/// Wrapper around `hb_face_t`.
pub struct Face<T> {
    raw: *mut sys::hb_face_t,
    phantom: PhantomData<T>,
}

impl Face<Owned> {
    /// Creates an empty font face.
    pub fn new() -> Self {
        unsafe { Face::from_raw(sys::hb_face_get_empty()) }
    }
}

impl<T> Face<T> {
    /// Create a `Face` from a raw pointer.
    ///
    /// # Safety
    ///
    /// Among other things, it is up to the caller to ensure that the type `T` matches the
    /// ownership semantics used.
    pub unsafe fn from_raw(raw: *mut sys::hb_face_t) -> Face<T> {
        Face {
            raw,
            phantom: PhantomData,
        }
    }

    /// Get a pointer to the underlying HarfBuzz face type.
    pub fn as_raw(&self) -> *mut sys::hb_face_t {
        self.raw
    }

    /// Creates the font face in HarfBuzz.
    ///
    /// Wrapper for `hb_face_create`. The index defaults to 0.
    pub fn from_blob(blob: Blob<T>) -> Face<T> {
        Face::new_with_index(blob, 0)
    }

    /// Creates the font face in HarfBuzz.
    ///
    /// Wrapper for `hb_face_create`.
    fn new_with_index(blob: Blob<T>, index: c_uint) -> Face<T> {
        unsafe {
            let raw = sys::hb_face_create(blob.as_raw(), index);
            // `hb_face_t` increments the reference count to the blob, so the original `Blob` is
            // still valid.
            Face::from_raw(raw)
        }
    }
}

impl<T: Clone> Clone for Face<T> {
    fn clone(&self) -> Self {
        unsafe {
            sys::hb_face_reference(self.raw);
            Face::from_raw(self.raw)
        }
    }
}

impl<T> Drop for Face<T> {
    fn drop(&mut self) {
        unsafe {
            sys::hb_face_destroy(self.raw);
        }
    }
}

impl From<Vec<u8>> for Face<Owned> {
    fn from(data: Vec<u8>) -> Self {
        let blob = Blob::from(data);
        Face::from_blob(blob)
    }
}

impl From<Arc<Vec<u8>>> for Face<Owned> {
    fn from(data: Arc<Vec<u8>>) -> Self {
        let blob = Blob::from(data);
        Face::from_blob(blob)
    }
}

impl<'a> From<&'a [u8]> for Face<Borrowed<'a>> {
    fn from(data: &'a [u8]) -> Self {
        let blob = Blob::from(data);
        Face::from_blob(blob)
    }
}
