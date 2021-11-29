use alloc::{string::String, vec::Vec};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, Ordering};

pub(crate) const PROS_ERR: i32 = i32::MAX;
pub(crate) const PROS_ERR_U32: u32 = i32::MAX as u32;
pub(crate) const PROS_ERR_F: f64 = f64::INFINITY;

pub struct StaticMut<T> {
	has_init: AtomicBool,
	item: MaybeUninit<T>,
}

impl<T> StaticMut<T> {
	pub const fn new() -> Self {
		Self {
			has_init: AtomicBool::new(false),
			item: MaybeUninit::uninit(),
		}
	}

	/// This function will only ever be called once
	pub fn call_once<F: FnOnce() -> T>(&self, f: F) {
		let s = unsafe { &mut *(self as *const Self as *mut Self) };

		if s.has_init.load(Ordering::SeqCst) {
			return;
		}
		unsafe {
			s.item.as_mut_ptr().write(f());
		}
		s.has_init.store(true, Ordering::SeqCst);
	}

	/// Wait for self.item to be set to something with a spinlock
	pub fn wait(&self) -> &mut T {
		let s = unsafe { &mut *(self as *const Self as *mut Self) };

		while !s.has_init.load(Ordering::Relaxed) {}
		unsafe { &mut *s.item.as_mut_ptr() }
	}
}

extern "C" {
	// Returns a pointer to this threads errno value
	fn __errno() -> *mut i32;
}

pub(crate) fn get_errno() -> libc::c_int {
	unsafe { *__errno() }
}

pub fn cstring_from(cstr: *const libc::c_char) -> String {
	unsafe {
		String::from_utf8_lossy(core::slice::from_raw_parts(
			cstr as *const u8,
			libc::strnlen(cstr, 512),
		))
		.into()
	}
}

pub fn to_cstring(s: String) -> Vec<u8> {
	let mut bytes = s.into_bytes();
	bytes.push(0);
	bytes
}
