pub mod controller;
pub mod motor;

use crate::util::get_errno;

/// Possible errors that could be returned from components with simple return type
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
