#![no_std]

use core::ffi::c_void;

extern crate libc;

extern "C" {
    pub fn iso_calloc(nmb: usize, size: usize) -> *mut c_void;
    pub fn iso_alloc(size: usize) -> *mut c_void;
    pub fn iso_realloc(p: *mut c_void, size: usize) -> *mut c_void;
    pub fn iso_free(p: *mut c_void);
}
