#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod bindings {
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod macros;
pub mod robot;
pub mod util;

pub mod motor;
pub mod rtos;

pub mod prelude {
	pub use crate::robot::Robot;
	pub use libc_print::std_name::*;
}

// LANGUAGE ITEMS
use core::alloc::{GlobalAlloc, Layout};

struct LibcAlloc;
unsafe impl GlobalAlloc for LibcAlloc {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		libc::memalign(layout.align(), layout.size()) as *mut u8
	}
	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
		libc::free(ptr as *mut core::ffi::c_void)
	}
}

#[global_allocator]
static ALLOC: LibcAlloc = LibcAlloc;

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
	panic!("alloc failed: {:?}", layout);
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	libc_print::libc_eprintln!("panic has occured: {:?}", info);

	unsafe {
		libc::_exit(1);
	}
}
