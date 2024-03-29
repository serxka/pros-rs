use alloc::{string::String, vec::Vec};

pub const PROS_ERR: i32 = i32::MAX;
pub const PROS_ERR_U32: u32 = i32::MAX as u32;
pub const PROS_ERR_F: f64 = f64::INFINITY;
pub const PROS_ERR_VISION_OBJECT_SIG: u8 = 255;

pub fn get_errno() -> libc::c_int {
	unsafe { *crate::bindings::__errno() }
}

#[allow(unused)]
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
	bytes.reserve(bytes.len() + 1);
	bytes.push(0);
	bytes
}
