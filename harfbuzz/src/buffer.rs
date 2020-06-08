// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module wraps the `hb_buffer_t` type and provides ancilliary types and functions.

use std::{
    convert::TryInto,
    os::raw::{c_char, c_int},
    ptr,
};
use sys;

use crate::{Direction, Language, Script};

/// A series of Unicode characters.
///
/// ## Adding Text
///
/// Since in Rust, a value of type `&str` must contain valid UTF-8
/// text, adding text to a `Buffer` is simple:
///
/// ```
/// # use harfbuzz::Buffer;
/// let mut b = Buffer::new();
/// b.add_str("Hello World");
/// assert_eq!(b.is_empty(), false);
/// ```
///
/// or, more simply:
///
/// ```
/// # use harfbuzz::Buffer;
/// let b = Buffer::with("Hello World");
/// assert_eq!(b.is_empty(), false);
/// ```
///
/// ## Segment Properties
///
/// In addition to the text itself, there are three important properties
/// that influence how a piece of text is shaped:
///
/// * Direction: The direction in which the output glyphs flow. This is
///   typically left to right or right to left. This is controlled via
///   the [`set_direction`] method on `Buffer`.
/// * Script: Script is crucial for choosing the proper shaping behaviour
///   for scripts that require it (e.g. Arabic) and the which OpenType
///   features defined in the font to be applied. This is controlled via
///   the [`set_script`] method on `Buffer`.
/// * Language: Languages are crucial for selecting which OpenType feature
///   to apply to the buffer which can result in applying language-specific
///   behaviour. Languages are orthogonal to the scripts, and though they
///   are related, they are different concepts and should not be confused
///   with each other. This is controlled via the [`set_language`] method
///   on `Buffer`.
///
/// Additionally, Harfbuzz can attempt to infer the values for these
/// properties using the [`guess_segment_properties`] method on `Buffer`:
///
/// ```
/// # use harfbuzz::{Buffer, Direction, sys, Script};
/// let mut b = Buffer::with("مساء الخير");
/// b.guess_segment_properties();
/// assert_eq!(b.get_direction(), Direction::RTL);
/// assert_eq!(b.get_script(), Script::Arabic);
/// ```
///
/// [`set_direction`]: #method.set_direction
/// [`set_script`]: #method.set_script
/// [`set_language`]: #method.set_language
/// [`guess_segment_properties`]: #method.guess_segment_properties
pub struct Buffer {
    /// The underlying `hb_buffer_t` from the `harfbuzz-sys` crate.
    ///
    /// This isn't commonly needed unless interfacing directly with
    /// functions from the `harfbuzz-sys` crate that haven't been
    /// safely exposed.
    raw: *mut sys::hb_buffer_t,
}

impl Buffer {
    /// Create a new, empty buffer.
    ///
    /// ```
    /// # use harfbuzz::Buffer;
    /// let b = Buffer::new();
    /// assert!(b.is_empty());
    /// ```
    pub fn new() -> Self {
        Buffer::default()
    }

    /// Construct a `Buffer` from a raw pointer. Takes ownership of the buffer.
    pub unsafe fn from_raw(raw: *mut sys::hb_buffer_t) -> Self {
        Buffer { raw }
    }

    /// Borrows a raw pointer to the buffer.
    pub fn as_ptr(&self) -> *mut sys::hb_buffer_t {
        self.raw
    }

    /// Gives up ownership and returns a raw pointer to the buffer.
    pub fn into_raw(self) -> *mut sys::hb_buffer_t {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }

    /// Create a new buffer with the given text.
    pub fn with(text: &str) -> Self {
        let mut b = Buffer::new();
        b.add_str(text);
        b
    }

    /// Create a new, empty buffer with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut b = Buffer::default();
        b.reserve(capacity);
        b
    }

    /// Add UTF-8 encoded text to the buffer.
    pub fn add_str(&mut self, text: &str) {
        unsafe {
            sys::hb_buffer_add_utf8(
                self.raw,
                text.as_ptr() as *const c_char,
                text.len() as c_int,
                0,
                text.len() as c_int,
            )
        };
    }

    /// Append part of the contents of another buffer to this one.
    ///
    /// ```
    /// # use harfbuzz::Buffer;
    /// let mut b1 = Buffer::with("butter");
    /// let b2 = Buffer::with("fly");
    /// b1.append(&b2, 0, 3);
    /// assert_eq!(b1.len(), "butterfly".len());
    /// ```
    pub fn append(&mut self, other: &Buffer, start: usize, end: usize) {
        unsafe {
            sys::hb_buffer_append(
                self.raw,
                other.raw,
                start as std::os::raw::c_uint,
                end as std::os::raw::c_uint,
            )
        };
    }

    /// Throw away text stored in the buffer, but maintain the
    /// currently configured Unicode functions and flags.
    ///
    /// Text, glyph info, and segment properties will be discarded.
    pub fn clear_contents(&mut self) {
        unsafe { sys::hb_buffer_clear_contents(self.raw) };
    }

    /// Throw away all data stored in the buffer as well as configuration
    /// parameters like Unicode functions, flags, and segment properties.
    pub fn reset(&mut self) {
        unsafe { sys::hb_buffer_reset(self.raw) };
    }

    /// Preallocate space to fit at least *size* number of items.
    ///
    /// FIXME: Does this correctly match the expected semantics?
    pub fn reserve(&mut self, size: usize) {
        unsafe { sys::hb_buffer_pre_allocate(self.raw, size as u32) };
    }

    /// Returns the number of elements in the buffer, also referred to as its 'length'.
    pub fn len(&self) -> usize {
        unsafe { sys::hb_buffer_get_length(self.raw) as usize }
    }

    /// Returns `true` if the buffer contains no data.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Sets unset buffer segment properties based on buffer Unicode
    /// contents.
    ///
    /// If buffer is not empty, it must have content type
    /// `HB_BUFFER_CONTENT_TYPE_UNICODE`.
    ///
    /// If buffer script is not set (ie. is `HB_SCRIPT_INVALID`), it will
    /// be set to the Unicode script of the first character in the buffer
    /// that has a script other than `HB_SCRIPT_COMMON`,
    /// `HB_SCRIPT_INHERITED`, and `HB_SCRIPT_UNKNOWN`.
    ///
    /// Next, if buffer direction is not set (ie. is `Direction::Invalid`),
    /// it will be set to the natural horizontal direction of the buffer
    /// script as returned by `hb_script_get_horizontal_direction()`.
    ///
    /// Finally, if buffer language is not set (ie. is `HB_LANGUAGE_INVALID`),
    /// it will be set to the process's default language as returned by
    /// `hb_language_get_default()`. This may change in the future by
    /// taking buffer script into consideration when choosing a language.
    ///
    /// ```
    /// # use harfbuzz::{Buffer, Direction, sys, Script};
    /// let mut b = Buffer::with("Hello, world!");
    /// b.guess_segment_properties();
    /// assert_eq!(b.get_direction(), Direction::LTR);
    /// assert_eq!(b.get_script(), Script::Latin);
    /// ```
    ///
    /// See also:
    ///
    /// * [`get_direction`](#method.get_direction)
    /// * [`set_direction`](#method.set_direction)
    /// * [`get_script`](#method.get_script)
    /// * [`set_script`](#method.set_script)
    /// * [`get_language`](#method.get_language)
    /// * [`set_language`](#method.set_language)
    pub fn guess_segment_properties(&mut self) {
        unsafe { sys::hb_buffer_guess_segment_properties(self.raw) };
    }

    /// Set the text flow direction of the buffer.
    ///
    /// No shaping can happen without setting buffer direction, and
    /// it controls the visual direction for the output glyphs; for
    /// RTL direction the glyphs will be reversed. Many layout features
    /// depend on the proper setting of the direction, for example,
    /// reversing RTL text before shaping, then shaping with LTR direction
    /// is not the same as keeping the text in logical order and shaping
    /// with RTL direction.
    ///
    /// See also:
    ///
    /// * [`get_direction`](#method.get_direction)
    /// * [`guess_segment_properties`](#method.guess_segment_properties)
    pub fn set_direction(&mut self, direction: Direction) {
        unsafe { sys::hb_buffer_set_direction(self.raw, direction.into()) };
    }

    /// Get the text flow direction for the buffer.
    ///
    /// See also:
    ///
    /// * [`set_direction`](#method.set_direction)
    pub fn get_direction(&self) -> Direction {
        (unsafe { sys::hb_buffer_get_direction(self.raw) }).into()
    }

    /// Sets the script of buffer to *script*.
    ///
    /// Script is crucial for choosing the proper shaping behaviour
    /// for scripts that require it (e.g. Arabic) and the which
    /// OpenType features defined in the font to be applied.
    ///
    /// See also:
    ///
    /// * [`get_script`](#method.get_script)
    /// * [`guess_segment_properties`](#method.guess_segment_properties)
    pub fn set_script(&mut self, script: Script) {
        unsafe { sys::hb_buffer_set_script(self.raw, script.as_raw()) };
    }

    /// Get the script for the buffer.
    ///
    /// See also:
    ///
    /// * [`set_script`](#method.set_script)
    pub fn get_script(&self) -> Script {
        unsafe { Script::from_raw(sys::hb_buffer_get_script(self.raw)) }
    }

    /// Sets the language of buffer to *language*.
    ///
    /// Languages are crucial for selecting which OpenType feature
    /// to apply to the buffer which can result in applying
    /// language-specific behaviour. Languages are orthogonal to
    /// the scripts, and though they are related, they are different
    /// concepts and should not be confused with each other.
    ///
    /// See also:
    ///
    /// * [`get_language`](#method.get_language)
    /// * [`guess_segment_properties`](#method.guess_segment_properties)
    pub fn set_language(&mut self, language: Language) {
        unsafe { sys::hb_buffer_set_language(self.raw, language.as_raw()) };
    }

    /// Get the language for the buffer.
    ///
    /// See also:
    ///
    /// * [`set_language`](#method.set_language)
    pub fn get_language(&self) -> Language {
        unsafe { Language::from_raw(sys::hb_buffer_get_language(self.raw)) }
    }

    /// Gets the current content type of the buffer.
    pub fn content_type(&self) -> ContentType {
        unsafe { ContentType::from_raw(sys::hb_buffer_get_content_type(self.raw)) }
    }

    /// Get the currently set segment properties.
    pub fn segment_properties(&self) -> SegmentProperties {
        unsafe {
            let mut props = SegmentProperties::uninit();
            sys::hb_buffer_get_segment_properties(
                self.raw,
                &mut props as *mut sys::hb_segment_properties_t,
            );
            SegmentProperties::from_raw(props)
        }
    }

    /// Get the currently set segment properties.
    pub fn set_segment_properties(&mut self, props: SegmentProperties) {
        unsafe {
            sys::hb_buffer_set_segment_properties(
                self.raw,
                &props.as_raw() as *const sys::hb_segment_properties_t,
            )
        }
    }

    /// Get the glyphs and their positions
    pub fn glyphs<'a>(&'a self) -> Option<Glyphs<'a>> {
        if self.content_type() != ContentType::Glyphs {
            return None;
        }
        unsafe {
            let mut infos_len = 0;
            let infos = sys::hb_buffer_get_glyph_infos(self.raw, &mut infos_len);
            let mut positions_len = 0;
            let positions = sys::hb_buffer_get_glyph_positions(self.raw, &mut positions_len);
            assert_eq!(infos_len, positions_len);
            Some(Glyphs {
                infos: std::slice::from_raw_parts(infos, infos_len.try_into().unwrap()),
                // repr(transparent) means we can transmute
                positions: std::slice::from_raw_parts(
                    positions as *const GlyphPosition,
                    positions_len.try_into().unwrap(),
                ),
            })
        }
    }
}

impl std::fmt::Debug for Buffer {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Buffer")
            .field("direction", &self.get_direction())
            .field("script", &self.get_script())
            .field("language", &self.get_language())
            .finish()
    }
}

impl Default for Buffer {
    /// Create a new, empty buffer.
    fn default() -> Self {
        Buffer {
            raw: unsafe { sys::hb_buffer_create() },
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { sys::hb_buffer_destroy(self.raw) }
    }
}

/// The different states a `Buffer` might be in.
///
/// The contents of a `Buffer` have different meanings depending on the buffer state. When it is
/// create it will be invalid, once text has been added it will be unicode, and then once a shaping
/// run has happened it will be glyph.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ContentType {
    /// The buffer has not yet been loaded with text.
    Invalid,
    /// The buffer contains text and is ready for shaping.
    Unicode,
    /// Shaping has run and the buffer contains glyph positions.
    Glyphs,
}

impl ContentType {
    /// Get `ContentType` from the respect `hb_buffer_content_type_t`.
    #[inline]
    pub fn from_raw(raw: sys::hb_buffer_content_type_t) -> Self {
        match raw {
            sys::HB_BUFFER_CONTENT_TYPE_INVALID => ContentType::Invalid,
            sys::HB_BUFFER_CONTENT_TYPE_UNICODE => ContentType::Unicode,
            sys::HB_BUFFER_CONTENT_TYPE_GLYPHS => ContentType::Glyphs,
            _ => panic!("unexpected content type"),
        }
    }

    /// Get `ContentType` from the respect `hb_buffer_content_type_t`.
    #[inline]
    pub fn into_raw(self) -> sys::hb_buffer_content_type_t {
        match self {
            ContentType::Invalid => sys::HB_BUFFER_CONTENT_TYPE_INVALID,
            ContentType::Unicode => sys::HB_BUFFER_CONTENT_TYPE_UNICODE,
            ContentType::Glyphs => sys::HB_BUFFER_CONTENT_TYPE_GLYPHS,
        }
    }
}

/// Properties for a shaping run (specifically language, script, and direction).
///
/// Can be used with `set_segment_properties` to avoid repetitive setting of individual properties.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct SegmentProperties {
    raw: sys::hb_segment_properties_t,
}

impl SegmentProperties {
    fn uninit() -> sys::hb_segment_properties_t {
        sys::hb_segment_properties_t {
            direction: 0,
            script: 0,
            language: ptr::null_mut(),
            reserved1: ptr::null_mut(),
            reserved2: ptr::null_mut(),
        }
    }

    /// Go from `sys::hb_segment_properties_t` to `SegmentProperites`.
    fn from_raw(raw: sys::hb_segment_properties_t) -> Self {
        SegmentProperties { raw }
    }

    /// Go from `SegmentProperites` to `sys::hb_segment_properties_t`.
    fn as_raw(self) -> sys::hb_segment_properties_t {
        self.raw
    }

    /// The direction set in this segment properties.
    pub fn direction(&self) -> Direction {
        self.raw.direction.into()
    }

    /// Sets the direction in this segment properties.
    pub fn set_direction(&mut self, direction: Direction) {
        self.raw.direction = direction.into();
    }

    /// The script set in this segment properties.
    pub fn script(&self) -> Script {
        Script::from_raw(self.raw.script)
    }

    /// Sets the script in this segment properties.
    pub fn set_script(&mut self, script: Script) {
        self.raw.script = script.as_raw();
    }

    /// The language set in this segment properties.
    pub fn language(&self) -> Language {
        unsafe { Language::from_raw(self.raw.language) }
    }

    /// Sets the language in this segment properties.
    pub fn set_language(&mut self, language: Language) {
        self.raw.language = language.as_raw();
    }
}

pub struct Glyphs<'a> {
    indexes: &'a [sys::hb_glyph_info_t],
    positions: &'a [GlyphPositions],
}

impl<'a> Glyphs<'a> {
    /// Get the underlying info object from which we get the glyph index.
    pub fn infos(&self) -> &'a [sys::hb_glyph_info_t] {
        self.infos
    }

    /// Get the positions to draw the glyphs.
    pub fn positions(&self) -> &'a [GlyphPosition] {
        self.positions
    }

    /// Iterate through the glyphs to draw.
    pub fn iter(&self) -> impl Iterator<Item = Glyph> + 'a {
        self.infos
            .iter()
            .zip(self.positions.iter())
            .map(|(info, position)| Glyph {
                index: info.codepoint,
                position,
            })
    }
}

pub struct Glyph {
    /// The index of the glyph in the font.
    pub index: u32,
    /// The position (offsets) to draw the glyph at, and the amount to move the cursor by after.
    pub position: GlyphPosition,
}

#[repr(transparent)]
pub struct GlyphPosition {
    raw: sys::hb_glyph_position_t,
}

impl GlyphPosition {
    /// The amount to move the cursor accross after drawing this glyph in horizontal shaping.
    pub fn x_advance(&self) -> i32 {
        self.raw.x_advance
    }

    /// The amount to move the cursor down/up after drawing this glyph in vertical shaping.
    pub fn y_advance(&self) -> i32 {
        self.raw.y_advance
    }

    /// The amount to offset from the cursor before starting to draw the glyph (in the x axis).
    pub fn x_offset(&self) -> i32 {
        self.raw.x_offset
    }

    /// The amount to offset from the cursor before starting to draw the glyph (in the y axis).
    pub fn y_offset(&self) -> i32 {
        self.raw.y_offset
    }
}
