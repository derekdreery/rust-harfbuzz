//! The `Face` type and associated types/functions.
//!
//! A `Face` is a font face stored in a raw encoded format (currently always OpenType).
use crate::{blob::Blob, decompose_tag, font::Font, sys, Borrowed, Owned, Ownership};
use std::{convert::TryInto, iter, marker::PhantomData, sync::Arc};

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

    fn has_variation_data(&self) -> bool {
        unsafe { sys::hb_ot_var_has_data(self.raw) != 0 }
    }

    /// Extract opentype variation data, if it is present.
    pub fn variation_data(&self) -> Option<Vec<AxisInfo>> {
        if !self.has_variation_data() {
            return None;
        }
        unsafe {
            let mut count = sys::hb_ot_var_get_axis_count(self.raw);
            // We have to allocate the infos type, which will then by populated by harfbuzz.
            let mut data: Vec<AxisInfo> = iter::repeat(Default::default())
                .take(count.try_into().unwrap())
                .collect();
            sys::hb_ot_var_get_axis_infos(
                self.raw,
                0,
                &mut count,
                data.as_mut_ptr() as *mut sys::hb_ot_var_axis_info_t,
            );
            // If for some reason there were fewer axes than we expected, truncate.
            let count: usize = count.try_into().unwrap();
            if count < data.len() {
                data.truncate(count)
            }
            Some(data)
        }
    }
}

impl<T: Ownership> Clone for Face<T> {
    fn clone(&self) -> Self {
        unsafe {
            sys::hb_face_make_immutable(self.raw);
            Face::from_raw(sys::hb_face_reference(self.raw))
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

/// This struct contains information on a particular axis of variation within a font.
///
/// Examples of variation axes include: weight (bold, light), slant, italic, width, ...
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct AxisInfo {
    raw: sys::hb_ot_var_axis_info_t,
}

impl AxisInfo {
    /// Get the index of the axis in the font.
    ///
    /// This is used when setting the variations for a font.
    pub fn axis_index(&self) -> u32 {
        self.raw.axis_index
    }

    /// Get the tag (a 4 byte identifier) for the variation axis.
    #[inline]
    pub fn tag(&self) -> [u8; 4] {
        decompose_tag(self.raw.tag)
    }

    /// The smallest value for the axis that the font can use.
    pub fn min_value(&self) -> f32 {
        self.raw.min_value
    }

    /// The value that this axis will be set to when a font is created.
    pub fn default_value(&self) -> f32 {
        self.raw.default_value
    }

    /// The largest value for the axis that the font can use.
    pub fn max_value(&self) -> f32 {
        self.raw.max_value
    }
}

/// The standard tag for the italic axis.
pub const ITALIC_TAG: &[u8; 4] = b"ital";
/// The standard tag for the optical size axis.
pub const OPTICAL_SIZE_TAG: &[u8; 4] = b"opsz";
/// The standard tag for the slant axis.
pub const SLANT_TAG: &[u8; 4] = b"slnt";
/// The standard tag for the width axis.
pub const WIDTH_TAG: &[u8; 4] = b"wdth";
/// The standard tag for the weight axis.
pub const WEIGHT_TAG: &[u8; 4] = b"wght";

impl Default for AxisInfo {
    fn default() -> Self {
        Self {
            raw: sys::hb_ot_var_axis_info_t {
                axis_index: 0,
                tag: 0,
                name_id: sys::HB_OT_NAME_ID_INVALID,
                flags: 0,
                min_value: 0.0,
                default_value: 0.0,
                max_value: 0.0,
                reserved: 0,
            },
        }
    }
}
