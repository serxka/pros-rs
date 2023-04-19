use alloc::{string::String, vec::Vec};

pub const PROS_ERR: i32 = i32::MAX;
pub const PROS_ERR_U32: u32 = i32::MAX as u32;
pub const PROS_ERR_F: f64 = f64::INFINITY;
pub const PROS_ERR_VISION_OBJECT_SIG: u8 = 255;

extern "C" {
	// Returns a pointer to this threads errno value
	fn __errno() -> *mut i32;
}

pub fn get_errno() -> libc::c_int {
	unsafe { *__errno() }
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

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Colour(u32);

impl Colour {
	pub const WHITE: Self = Self::new(0xFF, 0xFF, 0xFF);
	pub const RED: Self = Self::new(0xFF, 0x00, 0x00);
	pub const GREEN: Self = Self::new(0x00, 0xFF, 0x00);
	pub const BLUE: Self = Self::new(0x00, 0x00, 0xFF);

	const R_MASK: u32 = 0x00_FF_00_00;
	const G_MASK: u32 = 0x00_00_FF_00;
	const B_MASK: u32 = 0x00_00_00_FF;
	const R_OFFSET: u32 = 16;
	const G_OFFSET: u32 = 8;
	const B_OFFSET: u32 = 0;

	#[inline]
	pub const fn new(r: u8, g: u8, b: u8) -> Self {
		Colour(0).set_r(r).set_g(g).set_b(b)
	}

	#[inline]
	pub const fn as_u32(&self) -> u32 {
		self.0
	}

	#[inline]
	pub const fn set_r(self, r: u8) -> Self {
		Self((self.0 & !Self::R_MASK) | r as u32)
	}

	#[inline]
	pub const fn set_g(self, g: u8) -> Self {
		Self((self.0 & !Self::G_MASK) | g as u32)
	}

	#[inline]
	pub const fn set_b(self, b: u8) -> Self {
		Self((self.0 & !Self::B_MASK) | b as u32)
	}

	#[inline]
	pub const fn get_r(self) -> u8 {
		((self.0 & Self::R_MASK) >> Self::R_OFFSET) as u8
	}

	#[inline]
	pub const fn get_g(self) -> u8 {
		((self.0 & Self::G_MASK) >> Self::G_OFFSET) as u8
	}

	#[inline]
	pub const fn get_b(self) -> u8 {
		((self.0 & Self::B_MASK) >> Self::B_OFFSET) as u8
	}
}
