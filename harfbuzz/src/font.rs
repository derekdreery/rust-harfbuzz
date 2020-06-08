//! The `Font` type and associated types/functions.
//!
//! A `Font` is a font face along with any variation settings. It is used in shaping.

use crate::{
    compose_tag, decompose_tag,
    face::{Face, ITALIC_TAG, OPTICAL_SIZE_TAG, SLANT_TAG, WEIGHT_TAG, WIDTH_TAG},
    sys, Borrowed, Ownership,
};
use std::{convert::TryInto, fmt, marker::PhantomData};

/// Wrapper around `hb_font_t`.
///
/// Fonts are much more lightweight than `Face`s, but it can still be worth caching them. `Font`s
/// support multiple ownership through reference counting - `clone`s are cheap.
pub struct Font<T> {
    raw: *mut sys::hb_font_t,
    phantom: PhantomData<T>,
}

impl<T: Ownership> Font<T> {
    /// Creates a font from a face.
    #[inline]
    pub fn new(face: &Face<T>) -> Font<T> {
        unsafe { Font::from_raw(sys::hb_font_create(face.as_raw())) }
    }

    /// Create a `Font` from a raw pointer.
    ///
    /// # Safety
    ///
    /// Among other things, it is up to the caller to ensure that the type `T` matches the
    /// ownership semantics used.
    #[inline]
    pub unsafe fn from_raw(raw: *mut sys::hb_font_t) -> Font<T> {
        Font {
            raw,
            phantom: PhantomData,
        }
    }

    /// Get a pointer to the underlying HarfBuzz font.
    #[inline]
    pub fn as_raw(&self) -> *mut sys::hb_font_t {
        self.raw
    }

    /// Create a `Font` object by parsing the contents of this `Face`. `Font`s are much more
    /// lightweight than `Face`s but can still be cached if necessary.
    #[inline]
    pub fn face<'a>(&'a self) -> Face<Borrowed<'a>> {
        unsafe { Face::from_raw(sys::hb_font_get_face(self.raw)) }
    }

    /// Gets the width, height in pixels of the letter `m` with the current size.
    #[inline]
    pub fn pixels_per_em(&self) -> (u32, u32) {
        let mut x_ppem: u32 = 0;
        let mut y_ppem: u32 = 0;
        unsafe { sys::hb_font_get_ppem(self.raw, &mut x_ppem, &mut y_ppem) };
        (x_ppem, y_ppem)
    }

    /// Sets the size of the font, by specifying the width and height in pixels of the letter `m`.
    #[inline]
    pub fn set_pixels_per_em(&self, x_ppem: u32, y_ppem: u32) {
        unsafe { sys::hb_font_set_ppem(self.raw, x_ppem, y_ppem) }
    }

    /// Sets the size of the font, by specifying the width and height in pixels of the letter `m`.
    #[inline]
    pub fn with_pixels_per_em(self, x_ppem: u32, y_ppem: u32) -> Self {
        self.set_pixels_per_em(x_ppem, y_ppem);
        self
    }

    /// Gets the number of 'points' in the width of 'm'. There are 72 points in an inch.
    #[inline]
    pub fn point_size(&self) -> f32 {
        unsafe { sys::hb_font_get_ptem(self.raw) }
    }

    /// Sets the size of the font, by specifying the size in 'points' of the width of 'm'. There
    /// are 72 points in an inch.
    #[inline]
    pub fn set_point_size(&self, points: f32) {
        unsafe { sys::hb_font_set_ptem(self.raw, points) }
    }

    /// Sets the size of the font, by specifying the size in 'points' of the width of 'm'. There
    /// are 72 points in an inch.
    #[inline]
    pub fn with_point_size(self, points: f32) -> Self {
        self.set_point_size(points);
        self
    }

    /// Gets the subpixel scales in x and y directions.
    #[inline]
    pub fn scale(&self) -> (i32, i32) {
        let mut x_scale: i32 = 0;
        let mut y_scale: i32 = 0;
        unsafe { sys::hb_font_get_scale(self.raw, &mut x_scale, &mut y_scale) };
        (x_scale, y_scale)
    }

    /// Sets the subpixel scales in x and y directions.
    #[inline]
    pub fn set_scale(&self, x_scale: i32, y_scale: i32) {
        unsafe { sys::hb_font_set_scale(self.raw, x_scale, y_scale) }
    }

    /// Sets the subpixel scales in x and y directions.
    #[inline]
    pub fn with_scale(self, x_scale: i32, y_scale: i32) -> Self {
        self.set_scale(x_scale, y_scale);
        self
    }

    /// Sets variations for this font.
    ///
    /// Previous variations with the same tag name will be overwritten.
    #[inline]
    pub fn set_variations(&self, variations: &[Variation]) {
        unsafe {
            sys::hb_font_set_variations(
                self.raw,
                variations.as_ptr() as *mut sys::hb_variation_t,
                variations.len().try_into().unwrap(),
            )
        }
    }

    /// Sets variations for this font.
    ///
    /// Previous variations with the same tag name will be overwritten.
    #[inline]
    pub fn with_variations(self, variations: &[Variation]) -> Self {
        self.set_variations(variations);
        self
    }

    /// Create another font with exactly the same fontface and variations as `Self`.
    ///
    /// The two fonts can then be changed (e.g. by setting variations) without affecting each other.
    pub fn create_subfont(&self) -> Self {
        unsafe { Self::from_raw(sys::hb_font_create_sub_font(self.raw)) }
    }
}

impl<T: Ownership> Clone for Font<T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            sys::hb_font_make_immutable(self.raw);
            sys::hb_font_reference(self.raw);
            Font::from_raw(self.raw)
        }
    }
}

impl<T> Drop for Font<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            sys::hb_font_destroy(self.raw);
        }
    }
}

impl<'a, T: Ownership> From<&'a Face<T>> for Font<T> {
    #[inline]
    fn from(face: &'a Face<T>) -> Font<T> {
        Font::new(face)
    }
}

/// A variation - these can be applied to a font.
#[repr(transparent)]
pub struct Variation {
    raw: sys::hb_variation_t,
}

impl fmt::Debug for Variation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Variation")
            .field("tag", &self.tag())
            .field("value", &self.value())
            .finish()
    }
}

impl Variation {
    /// Create a new variation from its constituent parts.
    #[inline]
    pub fn new(tag: [u8; 4], value: f32) -> Self {
        Self {
            raw: sys::hb_variation_t {
                tag: compose_tag(tag),
                value,
            },
        }
    }

    /// Create a standard weight variation with the given value.
    ///
    /// A value of 400 is usually 'normal'.
    #[inline]
    pub fn weight(value: f32) -> Self {
        Self::new(*WEIGHT_TAG, value)
    }

    /// Create a standard italic variation with the given value.
    #[inline]
    pub fn italic(value: f32) -> Self {
        Self::new(*ITALIC_TAG, value)
    }

    /// Create a standard size variation with the given value.
    #[inline]
    pub fn size(value: f32) -> Self {
        Self::new(*OPTICAL_SIZE_TAG, value)
    }

    /// Create a standard slant variation with the given value.
    #[inline]
    pub fn slant(value: f32) -> Self {
        Self::new(*SLANT_TAG, value)
    }

    /// Create a standard slant variation with the given value.
    ///
    /// A value of 100 is usually 'normal'.
    #[inline]
    pub fn width(value: f32) -> Self {
        Self::new(*WIDTH_TAG, value)
    }

    /// Get the name of the variation axis this variation corresponds to.
    #[inline]
    pub fn tag(&self) -> [u8; 4] {
        decompose_tag(self.raw.tag)
    }

    /// Get the value to set the variation axis to.
    #[inline]
    pub fn value(&self) -> f32 {
        self.raw.value
    }
}
