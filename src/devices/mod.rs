//! All of the devices that have implemented interfaces in the PROS API.
//!
//! This includes controllers, motors and sensors. If you are looking to use the
//! TriPort expander you should look towards [`TriPort`][crate::ports::TriPort],
//! which works for internal TriPorts, and into the [`expander`] module for to
//! be able to create more TriPorts.

pub mod controller;
pub mod expander;
pub mod imu;
pub mod motor;
pub mod rotation;

use smallvec::SmallVec;

use crate::util::get_errno;

/// Possible errors that could be returned from devices in their operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceError {
	/// The current resource is currently being used by something else,
	ResourceInUse,
	/// The Port chosen is not within the range of supported ports of the
	/// V5 Brain,
	PortRange,
	/// The Port chosen cannot be configured as a motor,
	PortNotMotor,
	/// The port chosen cannot be configured as an IMU,
	PortNotIMU,
	/// The sensor is currently calibrating,
	StillCalibrating,
	/// The Port chosen cannot be configured as a rotation sensor,
	PortNotRotationSensor,
	/// An unknown error,
	#[doc(hidden)]
	Unknown(i32),
}

impl DeviceError {
	pub(crate) fn errno_generic() -> Self {
		match get_errno() {
			libc::EACCES => Self::ResourceInUse,
			libc::ENXIO => Self::PortRange,
			e => Self::Unknown(e),
		}
	}

	pub(crate) fn errno_motor() -> Self {
		match get_errno() {
			libc::ENODEV => Self::PortNotMotor,
			libc::ENXIO => Self::PortRange,
			e => Self::Unknown(e),
		}
	}

	pub(crate) fn errno_imu() -> Self {
		match get_errno() {
			libc::ENODEV => Self::PortNotIMU,
			libc::ENXIO => Self::PortRange,
			libc::EAGAIN => Self::StillCalibrating,
			e => Self::Unknown(e),
		}
	}

	pub(crate) fn errno_rotation() -> Self {
		match get_errno() {
			libc::ENODEV => Self::PortNotRotationSensor,
			libc::ENXIO => Self::PortRange,
			e => Self::Unknown(e),
		}
	}
}

/// Describes which direction the item is moving
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
	/// The item is moving in the positive direction, alternatively clockwise,
	Forward,
	/// The item is moving in the negative direction, alternatively
	/// anti-clockwise,
	Reverse,
}

use crate::ports::{Port, TriPort};
use controller::Controller;

/// A structure which represents all the possible devices connected to the V5
/// Brain.
pub struct Devices {
	/// Primary Controller
	pub master_controller: Option<Controller>,
	/// Secondary Controller
	pub slave_controller: Option<Controller>,
	pub ports: [Option<Port>; 21],
	pub triports: [Option<TriPort>; 8],
}

impl Devices {
	/// Unsafely constructs a new device holder. This is indented to only be
	/// made once by `pros-rs` and passed to
	/// [`Robot::new()`][crate::Robot::new()].
	///
	/// # Safety
	/// This function is unsafe as it uses Rusts ownership semantics to ensure
	/// that only **one** reference is ever held to a specific port. By creating
	/// more than one of these it is possible to have invalid handles to motors,
	/// sensors, etc. This would cause panics or undefined behaviour to occur in
	/// seemingly innocuous code.
	pub unsafe fn new() -> Self {
		let mut ports: SmallVec<[_; 21]> = SmallVec::new();
		for i in 1..22 {
			ports.push(Some(Port::new(i)));
		}

		let mut triports: SmallVec<[_; 8]> = SmallVec::new();
		for i in 1..9 {
			triports.push(Some(TriPort::new(i, None)));
		}

		Devices {
			master_controller: Some(Controller::master()),
			slave_controller: Some(Controller::slave()),
			ports: ports.into_inner().unwrap(),
			triports: triports.into_inner().unwrap(),
		}
	}
}
