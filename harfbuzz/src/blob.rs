// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::sys;
use std::marker::PhantomData;
use std::os::raw::{c_char, c_uint, c_void};
use std::sync::Arc;
use std::{mem, ops, ptr, slice};

// # Notes
//
//  - We don't need to implement `hb_blob_create_sub_blob`, because subslices can be got using
//    `Deref<[u8]>`.
//  - We don't implement `hb_blob_create_from_file` since `std` and the `mmap` crate handle file
//    loading better.

/// A marker struct to denote that data in a `Blob` is borrowed with some lifetime.
#[derive(Clone)]
pub struct Borrowed<'a> {
    phantom: PhantomData<&'a [u8]>,
}

/// A marker struct to denote that data in a `Blob` is mutably borrowed with some lifetime.
pub struct BorrowedMut<'a> {
    phantom: PhantomData<&'a mut [u8]>,
}

/// A marker struct to denote that data in a `Blob` is owned by the blob.
#[derive(Clone)]
pub struct Owned;

/// Blobs wrap a chunk of binary data to handle lifecycle management of data
/// while it is passed between client and HarfBuzz.
///
/// Blobs are primarily used to create font faces, but also to access font face
/// tables, as well as pass around other binary data.
pub struct Blob<T> {
    raw: *mut sys::hb_blob_t,
    phantom: PhantomData<T>,
}

impl Blob<Owned> {
    /// Create a blob wrapping an `Arc<Vec<u8>>`.
    ///
    /// This method allows creation of a blob without copying, where the
    /// data may be shared by Rust code and the blob. The `Vec` is freed
    /// when all references are dropped.
    ///
    /// ```
    /// # use std::sync::Arc;
    /// # use harfbuzz::Blob;
    /// let data = vec![1; 256];
    /// let blob = Blob::new_from_arc_vec(Arc::new(data));
    /// assert_eq!(blob.len(), 256);
    /// assert!(!blob.is_empty());
    /// ```
    pub fn from_arc(data: Arc<Vec<u8>>) -> Self {
        let len = data.len();
        assert!(len <= c_uint::max_value() as usize);
        unsafe {
            let data_ptr = data.as_ptr();
            let ptr = Arc::into_raw(data);

            // This has type hb_destroy_func_t
            unsafe extern "C" fn arc_vec_blob_destroy(user_data: *mut c_void) {
                drop(Arc::from_raw(user_data as *const Vec<u8>))
            }

            let hb_blob = sys::hb_blob_create(
                data_ptr as *const c_char,
                len as c_uint,
                sys::HB_MEMORY_MODE_READONLY,
                ptr as *mut c_void,
                Some(arc_vec_blob_destroy),
            );
            Blob::from_raw(hb_blob)
        }
    }

    /// Create a `Blob` from a `Vec`, taking ownership of the data.
    pub fn from_vec(data: Vec<u8>) -> Self {
        let len = data.len();
        let capacity = data.capacity();
        let ptr = data.as_ptr();
        assert!(len <= c_uint::max_value() as usize);
        unsafe {
            mem::forget(data);
            // If we could get the length from harfbuzz in the destroy function, we could avoid
            // this heap allocation by first converting the vec to `Box<[u8]>`.
            let destroy_data = Box::into_raw(Box::new((ptr, len, capacity)));

            // This has type hb_destroy_func_t
            unsafe extern "C" fn vec_blob_destroy(user_data: *mut c_void) {
                let (ptr, len, capacity) =
                    *Box::from_raw(user_data as *mut (*mut u8, usize, usize));
                drop(Vec::from_raw_parts(ptr, len, capacity))
            }

            let hb_blob = sys::hb_blob_create(
                ptr as *const c_char,
                len as c_uint,
                sys::HB_MEMORY_MODE_WRITABLE,
                destroy_data as *mut c_void,
                Some(vec_blob_destroy),
            );
            Blob::from_raw(hb_blob)
        }
    }
}

impl<'a> Blob<Borrowed<'a>> {
    /// Creates a blob by borrowing from rust data. The blob will be read-only.
    ///
    /// ```
    /// # use harfbuzz::Blob;
    /// let data = vec![1; 256];
    /// let blob = Blob::from_ref(&data);
    /// assert_eq!(blob.len(), 256);
    /// assert!(!blob.is_empty());
    /// ```
    pub fn from_ref(data: impl AsRef<[u8]> + 'a) -> Self {
        let data = data.as_ref();
        assert!(data.len() <= c_uint::max_value() as usize);
        unsafe {
            Blob::from_raw(sys::hb_blob_create(
                data.as_ptr() as *const c_char,
                data.len() as c_uint,
                sys::HB_MEMORY_MODE_READONLY,
                ptr::null_mut(), // user data
                None,            // destroy callback
            ))
        }
    }
}

impl<'a> Blob<BorrowedMut<'a>> {
    /// Creates a blob by mutably borrowing from rust data.
    ///
    /// Note that even though we only hold a single reference to the data, HarfBuzz might
    /// internally reference the data multiple times, meaning that mutations will fail.
    ///
    /// # Safety
    ///
    /// The reference must not be used elsewhere while the `Blob` is live.
    // TODO this might be safe, but needs some thought.
    pub unsafe fn from_mut(mut data: impl AsMut<[u8]> + 'a) -> Self {
        let data = data.as_mut();
        assert!(data.len() <= c_uint::max_value() as usize);
        Blob::from_raw(sys::hb_blob_create(
            data.as_ptr() as *const c_char,
            data.len() as c_uint,
            sys::HB_MEMORY_MODE_WRITABLE,
            ptr::null_mut(), // user data
            None,            // destroy callback
        ))
    }
}

impl<T> Blob<T> {
    /// Construct a `Blob` from a raw pointer. Takes ownership of the blob.
    pub unsafe fn from_raw(raw: *mut sys::hb_blob_t) -> Self {
        Blob {
            raw,
            phantom: PhantomData,
        }
    }

    /// Returns the size of the blob in bytes.
    pub fn len(&self) -> usize {
        unsafe { sys::hb_blob_get_length(self.raw) as usize }
    }

    /// Returns true if the length is zero.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Make this blob immutable.
    pub fn make_immutable(&self) {
        unsafe {
            // Even though this function mutates, it is safe because `Blob` is `!Send, Sync`
            // (meaning this function is atomic) and the mutation happens in C so no mutable
            // references are ever held.
            sys::hb_blob_make_immutable(self.raw);
        }
    }

    /// Returns true if the blob is immutable.
    pub fn is_immutable(&self) -> bool {
        unsafe { sys::hb_blob_is_immutable(self.raw) != 0 }
    }

    /// Borrows a raw pointer to the blob.
    pub fn as_raw(&self) -> *mut sys::hb_blob_t {
        self.raw
    }

    /// Gives up ownership and returns a raw pointer to the blob.
    pub fn into_raw(self) -> *mut sys::hb_blob_t {
        let raw = self.raw;
        mem::forget(self);
        raw
    }

    /// Create an owned blob, copying the underlying data.
    // TODO implement std::borrow::{Borrow, ToOwned} if possible.
    pub fn to_owned(&self) -> Blob<Owned> {
        let data: &[u8] = &**self;
        data.to_owned().into()
    }

    /// Wrapper for `hb_blob_reference`.
    unsafe fn reference(&self) -> *mut sys::hb_blob_t {
        sys::hb_blob_reference(self.as_raw())
    }
}

impl<T> Drop for Blob<T> {
    /// Decrement the reference count, and destroy the blob if the reference count is zero.
    fn drop(&mut self) {
        unsafe {
            sys::hb_blob_destroy(self.raw);
        }
    }
}

impl<T: Clone> Clone for Blob<T> {
    fn clone(&self) -> Self {
        unsafe {
            self.make_immutable();
            Blob::from_raw(self.reference())
        }
    }
}

impl From<Vec<u8>> for Blob<Owned> {
    fn from(data: Vec<u8>) -> Self {
        Blob::from_vec(data)
    }
}

impl From<Arc<Vec<u8>>> for Blob<Owned> {
    fn from(data: Arc<Vec<u8>>) -> Self {
        Blob::from_arc(data)
    }
}

impl<'a> From<&'a [u8]> for Blob<Borrowed<'a>> {
    fn from(slice: &'a [u8]) -> Self {
        Blob::from_ref(slice)
    }
}

impl<T> ops::Deref for Blob<T> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = sys::hb_blob_get_data(self.raw, &mut len);
            assert!(!ptr.is_null(), "hb_blob_get_data failed");
            slice::from_raw_parts(ptr as *const u8, len as usize)
        }
    }
}

const DEREF_MUT_ERR: &str =
    "hb_blob_get_data_writable failed, possibly because the data is immutable";

impl<'a> ops::DerefMut for Blob<BorrowedMut<'a>> {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            let mut len = 0;
            let ptr = sys::hb_blob_get_data_writable(self.raw, &mut len);
            assert!(!ptr.is_null(), DEREF_MUT_ERR);
            slice::from_raw_parts_mut(ptr as *mut u8, len as usize)
        }
    }
}

impl<'a> ops::DerefMut for Blob<Owned> {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            let mut len = 0;
            let ptr = sys::hb_blob_get_data_writable(self.raw, &mut len);
            assert!(!ptr.is_null(), DEREF_MUT_ERR);
            slice::from_raw_parts_mut(ptr as *mut u8, len as usize)
        }
    }
}
