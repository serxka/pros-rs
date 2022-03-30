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
	pub port01: Option<Port>,
	pub port02: Option<Port>,
	pub port03: Option<Port>,
	pub port04: Option<Port>,
	pub port05: Option<Port>,
	pub port06: Option<Port>,
	pub port07: Option<Port>,
	pub port08: Option<Port>,
	pub port09: Option<Port>,
	pub port10: Option<Port>,
	pub port11: Option<Port>,
	pub port12: Option<Port>,
	pub port13: Option<Port>,
	pub port14: Option<Port>,
	pub port15: Option<Port>,
	pub port16: Option<Port>,
	pub port17: Option<Port>,
	pub port18: Option<Port>,
	pub port19: Option<Port>,
	pub port20: Option<Port>,
	pub port21: Option<Port>,
	pub port_a: Option<TriPort>,
	pub port_b: Option<TriPort>,
	pub port_c: Option<TriPort>,
	pub port_d: Option<TriPort>,
	pub port_e: Option<TriPort>,
	pub port_f: Option<TriPort>,
	pub port_g: Option<TriPort>,
	pub port_h: Option<TriPort>,
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
		Devices {
			master_controller: Some(Controller::master()),
			slave_controller: Some(Controller::slave()),
			port01: Some(Port::new(1)),
			port02: Some(Port::new(2)),
			port03: Some(Port::new(3)),
			port04: Some(Port::new(4)),
			port05: Some(Port::new(5)),
			port06: Some(Port::new(6)),
			port07: Some(Port::new(7)),
			port08: Some(Port::new(8)),
			port09: Some(Port::new(9)),
			port10: Some(Port::new(10)),
			port11: Some(Port::new(11)),
			port12: Some(Port::new(12)),
			port13: Some(Port::new(13)),
			port14: Some(Port::new(14)),
			port15: Some(Port::new(15)),
			port16: Some(Port::new(16)),
			port17: Some(Port::new(17)),
			port18: Some(Port::new(18)),
			port19: Some(Port::new(19)),
			port20: Some(Port::new(20)),
			port21: Some(Port::new(21)),
			port_a: Some(TriPort::new(1, None)),
			port_b: Some(TriPort::new(2, None)),
			port_c: Some(TriPort::new(3, None)),
			port_d: Some(TriPort::new(4, None)),
			port_e: Some(TriPort::new(5, None)),
			port_f: Some(TriPort::new(6, None)),
			port_g: Some(TriPort::new(7, None)),
			port_h: Some(TriPort::new(8, None)),
		}
	}
}
