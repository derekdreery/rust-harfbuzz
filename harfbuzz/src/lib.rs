// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! HarfBuzz is a text shaping engine. It solves the problem of selecting
//! and positioning glyphs from a font given a Unicode string.

#![warn(missing_docs)]
#![deny(
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

use std::{marker::PhantomData, ptr};

pub extern crate harfbuzz_sys as sys;

mod scripts;
pub use self::scripts::Script;

pub mod buffer;
pub use self::buffer::Buffer;

mod direction;
pub use self::direction::Direction;

mod language;
pub use self::language::Language;

mod blob;
pub use self::blob::Blob;

pub mod face;
pub use self::face::Face;

pub mod font;
pub use self::font::Font;

/// Shape a line of text (convert unicode characters to glyphs and positions).
pub fn shape<T: Ownership>(font: &Font<T>, buffer: &Buffer) {
    unsafe { sys::hb_shape(font.as_raw(), buffer.as_ptr(), ptr::null(), 0) }
}

fn compose_tag(t: [u8; 4]) -> u32 {
    ((t[0] as u32) << 24) + ((t[1] as u32) << 16) + ((t[2] as u32) << 8) + (t[3] as u32)
}

fn decompose_tag(t: u32) -> [u8; 4] {
    [
        ((t >> 24) & 0xff) as u8,
        ((t >> 16) & 0xff) as u8,
        ((t >> 8) & 0xff) as u8,
        (t & 0xff) as u8,
    ]
}

/// A marker struct to denote that data in a `Blob` is owned by the blob.
///
/// HarfBuzz will have been given the info needed to destroy the rust object owning the data when
/// the `Blob` is dropped.
pub struct Owned;

/// A marker struct to denote that data in a `Blob` is borrowed with some lifetime.
pub struct Borrowed<'a> {
    phantom: PhantomData<&'a [u8]>,
}

mod sealed {
    pub trait Sealed {}
}

/// A sealed marker trait to denote ownership semantics.
pub trait Ownership: sealed::Sealed {}

impl sealed::Sealed for Owned {}
impl Ownership for Owned {}
impl<'a> sealed::Sealed for Borrowed<'a> {}
impl<'a> Ownership for Borrowed<'a> {}
