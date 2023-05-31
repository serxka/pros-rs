//! All of the devices that have implemented interfaces in the PROS API.
//!
//! This includes controllers, motors and sensors. If you are looking to use the
//! TriPort expander you should look towards [`TriPort`][crate::ports::TriPort],
//! which works for internal TriPorts, and into the [`expander`] module for to
//! be able to create more TriPorts.

pub mod controller;
pub mod distance;
pub mod expander;
pub mod gps;
pub mod imu;
pub mod led;
pub mod motor;
pub mod rotation;
pub mod vision;

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
	/// The Port chosen cannot be configured as a distance sensor,
	PortNotDistance,
	/// The Port chosen cannot be configured as a motor,
	PortNotMotor,
	/// The port chosen cannot be configured as an IMU,
	PortNotIMU,
	/// The sensor is currently calibrating,
	StillCalibrating,
	/// The Port chosen cannot be configured as a rotation sensor,
	PortNotRotationSensor,
	/// The Port chosen cannot be configured as a vision sensor,
	PortNotVisionSensor,
	/// The Vision sensor failed for an unknown reason,
	VisionUnknown,
	/// The Vision sensor cannot see any other objects which meet the
	/// requirements,
	VisionObjectsDeficit,
	/// The port chosen cannot be configured as an ADI port,
	PortNotADI,
	/// The V5 Brain ran out of memory
	OutOfMemory,
	/// An unknown error,
	#[doc(hidden)]
	Unknown,
}

impl DeviceError {
	pub(crate) fn errno_generic() -> Self {
		match get_errno() {
			libc::EACCES => Self::ResourceInUse,
			libc::ENXIO => Self::PortRange,
			libc::ENOMEM => Self::OutOfMemory,
			e => {
				if cfg!(debug_assertions) {
					panic!("reached unknown error ({e})");
				}
				Self::Unknown
			}
		}
	}

	pub(crate) fn errno_distance() -> Self {
		match get_errno() {
			libc::ENODEV => Self::PortNotDistance,
			libc::ENXIO => Self::PortRange,
			e => {
				if cfg!(debug_assertions) {
					panic!("reached unknown error ({e})");
				}
				Self::Unknown
			}
		}
	}

	pub(crate) fn errno_motor() -> Self {
		match get_errno() {
			libc::ENODEV => Self::PortNotMotor,
			libc::ENXIO => Self::PortRange,
			e => {
				if cfg!(debug_assertions) {
					panic!("reached unknown error ({e})");
				}
				Self::Unknown
			}
		}
	}

	/// This also works for the GPS
	pub(crate) fn errno_imu() -> Self {
		match get_errno() {
			libc::ENODEV => Self::PortNotIMU,
			libc::ENXIO => Self::PortRange,
			libc::EAGAIN => Self::StillCalibrating,
			e => {
				if cfg!(debug_assertions) {
					panic!("reached unknown error ({e})");
				}
				Self::Unknown
			}
		}
	}

	pub(crate) fn errno_rotation() -> Self {
		match get_errno() {
			libc::ENODEV => Self::PortNotRotationSensor,
			libc::ENXIO => Self::PortRange,
			e => {
				if cfg!(debug_assertions) {
					panic!("reached unknown error ({e})");
				}
				Self::Unknown
			}
		}
	}

	pub(crate) fn errno_vision() -> Self {
		match get_errno() {
			libc::ENODEV => Self::PortNotVisionSensor,
			libc::ENXIO => Self::PortRange,
			libc::EHOSTDOWN | libc::EAGAIN => Self::VisionUnknown,
			libc::EDOM => Self::VisionObjectsDeficit,
			e => {
				if cfg!(debug_assertions) {
					panic!("reached unknown error ({e})");
				}
				Self::Unknown
			}
		}
	}

	pub(crate) fn errno_adi() -> Self {
		match get_errno() {
			libc::ENXIO => Self::PortRange,
			libc::EINVAL | libc::EADDRINUSE => Self::PortNotADI,
			e => {
				if cfg!(debug_assertions) {
					panic!("reached unknown error ({e})");
				}
				Self::Unknown
			}
		}
	}
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
/// Brain. You can either take these values directly from the [`Devices`] or
/// you can use the provided utility functions which will check for correct
/// port indices.
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
			ports.push(Some(Port::new(i).unwrap()));
		}

		let mut triports: SmallVec<[_; 8]> = SmallVec::new();
		for i in 1..9 {
			triports.push(Some(TriPort::new(i, None).unwrap()));
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
	/// # Errors
	/// May return a [`DeviceError::PortRange`] if the port index is out of
	/// range or a [`DeviceError::ResourceInUse`] if the port has already been
	/// taken.
	///
	/// # Debug Assertions
	/// Assertion that the port index with the valid range for the V5 Brain.
	///
	/// # Examples
	/// ```
	/// let port = devices.take_port(1).unwrap();
	/// assert_eq!(1, port.get());
	/// ```
	pub fn take_port(&mut self, index: usize) -> Result<Port, DeviceError> {
		let within = (1..=21).contains(&index);
		debug_assert!(
			within,
			"This port value is not within the range of 1..=21 ({})",
			index
		);

		if within {
			match self.ports[index - 1].take() {
				Some(p) => Ok(p),
				None => Err(DeviceError::ResourceInUse),
			}
		} else {
			Err(DeviceError::PortRange)
		}
	}

	/// Take a TriPort out of this [`Devices`] structure. The index passed to
	/// this function is the same as that of the port.
	///
	/// # Errors
	/// May return a [`DeviceError::PortRange`] if the port index is out of
	/// range or a [`DeviceError::ResourceInUse`] if the port has already been
	/// taken.
	///
	/// # Debug Assertions
	/// Assertions that the port index with the valid range for the V5 Brain.
	pub fn take_triport(&mut self, index: usize) -> Result<TriPort, DeviceError> {
		let within = (1..=8).contains(&index);
		debug_assert!(
			within,
			"This port value is not within the range of 1..=8 ({})",
			index
		);

		if within {
			match self.triports[index - 1].take() {
				Some(p) => Ok(p),
				None => Err(DeviceError::ResourceInUse),
			}
		} else {
			Err(DeviceError::PortRange)
		}
	}
}
