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
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        iso_free_size(ptr as *mut c_void, layout.size());
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, _: Layout, size: usize) -> *mut u8 {
        iso_realloc(ptr as *mut c_void, size) as *mut u8
    }
}

impl IsoAlloc {
    /// creates a block of nmemb * size also
    /// checks for overflow, similar as `realloc` otherwise.
    pub unsafe fn reallocarray(
        &self,
        ptr: *mut u8,
        _: Layout,
        nmemb: usize,
        size: usize,
    ) -> *mut u8 {
        iso_reallocarray(ptr as *mut c_void, nmemb, size) as *mut u8
    }

    /// `malloc_usable_size` like call.
    pub fn usable_size(&self, ptr: *mut u8) -> usize {
        unsafe { iso_chunksz(ptr as *mut c_void) }
    }

    /// checks zones coherency.
    pub fn verify_zones(&self) {
        unsafe { iso_verify_zones() };
    }

    /// fetches peak memory usage.
    pub fn mem_usage(&self) -> u64 {
        unsafe { iso_alloc_mem_usage() }
    }

    /// memory leaks.
    pub fn leaks(&self) -> u64 {
        unsafe { iso_alloc_detect_leaks() }
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
            p = IsoAlloc.realloc(p, l, 4096);
            IsoAlloc.dealloc(p, l);
        }
    }

    #[test]
    fn large_alloc() {
        unsafe {
            let l = Layout::from_size_align(1 << 20, 32).unwrap();
            let p = IsoAlloc.alloc(l);
            IsoAlloc.dealloc(p, l);
        }
    }

    #[test]
    fn realloc_array() {
        unsafe {
            let l = Layout::from_size_align(8, 8).unwrap();
            let mut p = core::ptr::null_mut();
            p = IsoAlloc.realloc(p, l, 1024);
            p = IsoAlloc.reallocarray(p, l, 2, 1024);
            IsoAlloc.dealloc(p, l);
        }
    }

    #[test]
    fn utils() {
        use core::convert::TryInto;
        unsafe {
            let l = Layout::from_size_align(8, 8).unwrap();
            let a = IsoAlloc.alloc(l);
            assert!(IsoAlloc.usable_size(a) >= l.size());
            let ta = IsoAlloc.mem_usage();
            assert!(ta > l.size().try_into().unwrap());
            let b = IsoAlloc.alloc(l);
            let tb = IsoAlloc.mem_usage();
            assert!(tb >= ta);
            IsoAlloc.dealloc(b, l);
            IsoAlloc.dealloc(a, l);
            assert_eq!(IsoAlloc.leaks(), 0u64);
            IsoAlloc.verify_zones();
        }
    }
}
