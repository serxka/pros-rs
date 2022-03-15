pub mod controller;
pub mod imu;
pub mod motor;
pub mod rotation;

use crate::util::get_errno;

/// Possible errors that could be returned from components with simple return
/// type
#[derive(Debug)]
pub enum GenericError {
	/// The current resource is currently being used by something else
	ResourceInUse,
	/// An unknown error
	#[doc(hidden)]
	Unknown(i32),
}

impl GenericError {
	pub(crate) fn errno() -> Self {
		match get_errno() {
			libc::EACCES => Self::ResourceInUse,
			e => Self::Unknown(e),
		}
	}
}

/// Describes which direction the item is moving
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
	/// The item is moving in the positive direction, alternatively clockwise
	Forward,
	/// The item is moving in the negative direction, alternatively
	/// anti-clockwise
	Reverse,
}
