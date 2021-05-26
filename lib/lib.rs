#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use libc::{memalign, free};

struct LibcAlloc;
unsafe impl GlobalAlloc for LibcAlloc {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		memalign(layout.align(), layout.size()) as *mut u8
	}
	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
		free(ptr as *mut core::ffi::c_void)
	}
}

#[global_allocator]
static ALLOC: LibcAlloc = LibcAlloc;

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
	panic!("alloc failed: {:?}", layout);
}



#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod bindings {
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod entry;

#[allow(dead_code)]
extern "C" fn initialize() {
	entry::initialize();
}

#[allow(dead_code)]
extern "C" fn disabled() {
	entry::disabled();
}

#[allow(dead_code)]
extern "C" fn competition_initialize() {
	entry::competition_initialize();
}

#[allow(dead_code)]
extern "C" fn autonomous() {
	entry::autonomous();
}

#[allow(dead_code)]
extern "C" fn opcontrol() {
	entry::opcontrol();
}