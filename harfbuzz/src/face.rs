use crate::{
    blob::{Blob, Borrowed, Owned, Ownership},
    font::Font,
    sys,
};
use std::{marker::PhantomData, sync::Arc};

/// Wrapper around `hb_face_t`.
///
/// In HarfBuzz, faces are heavyweight objects because they include all the raw font data in
/// whatever format is in use.
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

impl<T: Ownership> Face<T> {
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

    /// Creates a font face in HarfBuzz.
    ///
    /// The `Blob` should contain the raw font data, for example an opentype or truetype
    /// specification. Since TrueType and OpenType collections may contain more than 1 font, and
    /// this function defaults to the first font found in such cases, you will need to use
    /// `Face::from_blob_index` to select a later font in the collection. This is rare, however,
    /// as most font face definitions contain only 1 font.
    #[inline]
    pub fn from_blob(blob: &Blob<T>) -> Face<T> {
        Face::from_blob_index(blob, 0)
    }

    /// Creates the font face in HarfBuzz.
    ///
    /// Same as `from_blob` except it allows you to select a font other than the first in a
    /// collection. Usually you will want `Face::from_blob` or one of the `From` implementations.
    pub fn from_blob_index(blob: &Blob<T>, index: u32) -> Face<T> {
        unsafe {
            // We assume c_uint is u32. It is better that the lib does not compile than it compiles
            // will overflow problems coming from casting.
            let raw = sys::hb_face_create(blob.as_raw(), index);
            // `hb_face_create` increments the reference count to the blob, so the original `Blob`
            // is still valid and the reference count should be decremeneted when it goes out of
            // scope - it shouldn't be `forget`ted (forgot).
            Face::from_raw(raw)
        }
    }

    /// Get the number of glyphs in the font face.
    // hb_face_glyph_count returns a c_uint - which is at least 16 bits wide. I'm using 32 bits as
    // a catchall.
    pub fn glyph_count(&self) -> u32 {
        // this won't compile if c_uint != u32.
        unsafe { sys::hb_face_get_glyph_count(self.as_raw()) }
    }

    /// Create a `Font` object by parsing the contents of this `Face`. `Font`s are much more
    /// lightweight than `Face`s but can still be cached if necessary.
    pub fn font(&self) -> Font<T> {
        Font::new(self)
    }
}

impl<T: Ownership> Clone for Face<T> {
    fn clone(&self) -> Self {
        unsafe {
            sys::hb_face_make_immutable(self.raw);
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
        Face::from_blob(&blob)
    }
}

impl From<Arc<Vec<u8>>> for Face<Owned> {
    fn from(data: Arc<Vec<u8>>) -> Self {
        let blob = Blob::from(data);
        Face::from_blob(&blob)
    }
}

impl<'a> From<&'a [u8]> for Face<Borrowed<'a>> {
    fn from(data: &'a [u8]) -> Self {
        let blob = Blob::from(data);
        Face::from_blob(&blob)
    }
}
