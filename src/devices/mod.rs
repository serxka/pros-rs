//! All of the devices that have implemented interfaces in the PROS API.
//!
//! This includes controllers, motors and sensors. If you are looking to use the
//! TriPort expander you should look towards [`TriPort`][crate::ports::TriPort],
//! which works for internal TriPorts, and into the [`expander`] module for to
//! be able to create more TriPorts.

pub mod controller;
pub mod expander;
pub mod gps;
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

	/// This also works for the GPS
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
	/// List of all ports on the V5 Brain. This array is zero indexed, however
	/// the 0th index is port 1.
	pub ports: [Option<Port>; 21],
	/// List of all tri-ports on the V5 Brain. This array is zero indexed, with
	/// the 0th index being port A and 7th index being port H.
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

	/// Takes the master controller out of this [`Devices`] structure.
	///
	/// # Panics
	/// This function will panic if the master controller has already been
	/// taken.
	pub fn take_master_controller(&mut self) -> Controller {
		self.master_controller.take().unwrap()
	}

	/// Takes the slave controller out of this [`Devices`] structure.
	///
	/// # Panics
	/// This function will panic if the slave controller has already been taken.
	pub fn take_slave_controller(&mut self) -> Controller {
		self.slave_controller.take().unwrap()
	}

	/// Take a Port out of this [`Devices`] structure. The index passed to this
	/// function is the same as that of the port.
	///
	/// # Assertions
	/// Assertion that the port index with the valid range
	/// for the V5 Brain.
	///
	/// # Panics
	/// This function will panic if the port has already been taken.
	///
	/// # Examples
	/// ```
	/// let port = devices.take_port(1);
	/// assert_eq!(1, port.get());
	/// ```
	pub fn take_port(&mut self, index: usize) -> Port {
		assert!(
			(1..=21).contains(&index),
			"This port value is not within the range of 1..=21 ({})",
			index
		);
		self.ports[index - 1].take().unwrap()
	}

	/// Take a TriPort out of this [`Devices`] structure. The index passed to
	/// this function is the same as that of the port.
	///
	/// # Assertions
	/// Assertions that the port index with the valid range
	/// for the V5 Brain.
	///
	/// # Panics
	/// This function will panic if the port has already been taken.
	pub fn take_triport(&mut self, index: usize) -> TriPort {
		assert!(
			(1..=8).contains(&index),
			"This port value is not within the range of 1..=8 ({})",
			index
		);
		self.triports[index - 1].take().unwrap()
	}
}
