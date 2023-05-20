#![no_std]
extern crate libisoalloc_sys as ffi;

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use ffi::*;

pub struct IsoAlloc;

/// As part of isoalloc security's makeup, addresses are 8 bytes aligned.
unsafe impl GlobalAlloc for IsoAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        iso_alloc(layout.size()) as *mut u8
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        iso_calloc(1usize, layout.size()) as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        iso_free(ptr as *mut c_void);
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, _: Layout, size: usize) -> *mut u8 {
        iso_realloc(ptr as *mut c_void, size) as *mut u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reuse_mem() {
        unsafe {
            let l = Layout::from_size_align(1024, 8)
                .expect("should be able to work 8 with bytes alignment");
            let mut p = IsoAlloc.alloc_zeroed(l);
            IsoAlloc.dealloc(p, l);
            p = core::ptr::null_mut();
            p = IsoAlloc.realloc(p, l, 4096);
            IsoAlloc.dealloc(p, l);
        }
    }
}
