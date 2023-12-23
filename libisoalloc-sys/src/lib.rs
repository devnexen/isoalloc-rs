#![no_std]

use core::ffi::c_void;

extern crate libc;

extern "C" {
    // basic api
    pub fn iso_calloc(nmb: usize, size: usize) -> *mut c_void;
    pub fn iso_alloc(size: usize) -> *mut c_void;
    pub fn iso_realloc(p: *mut c_void, size: usize) -> *mut c_void;
    pub fn iso_free(p: *mut c_void);
    pub fn iso_free_size(p: *mut c_void, size: usize);
    // extra
    pub fn iso_chunksz(p: *mut c_void) -> usize;
    pub fn iso_alloc_mem_usage() -> u64;
    pub fn iso_alloc_detect_leaks() -> u64;
}
