//! Contains types for managing the ports on the V5 Brain.

use crate::bindings::*;
use crate::devices::{imu::*, motor::*, rotation::*, Direction};

use core::num::NonZeroU8;

/// An object of a port on the V5 Brain.
///
/// This is an object which should not be created by hand in most cases. It uses
/// ownership semantics at runtime to help make sure a port is not used for more
/// than one device.
#[derive(Debug)]
pub struct Port(NonZeroU8);

impl Port {
	/// Create a new reference to a port on the V5 Brain. There must only be one
	/// reference to this port at any time throughout the execution of the
	/// program.
	///
	/// This function should not generally be called, prefer to use the port
	/// supplied by [`Robot::new()`][crate::Robot::new()] in the
	/// [`Devices`][crate::devices::Devices] structure.
	///
	/// # Assertions
	/// This function will assert that the port is within the range of 1 to 21
	/// (inclusive).
	///
	/// # Safety
	/// The user must make sure that when calling this function to create a new
	/// port, there isn't already a port with the same index.
	pub unsafe fn new(port: u8) -> Self {
		assert!(
			(1..=21).contains(&port),
			"This port value is not within the range of 1..=21 ({})",
			port
		);
		Port(NonZeroU8::new_unchecked(port))
	}

	/// Get the value for the port as a `u8` value.
	#[inline]
	pub fn get(&self) -> u8 {
		self.0.get()
	}

	/// This function will return what is currently **plugged** into this port
	/// of the V5 Brain. This value may be different from the value PROS has the
	/// port registered as.
	#[inline]
	pub fn plugged_type(&self) -> DeviceType {
		unsafe { registry_get_plugged_type(self.0.get() - 1).into() }
	}

	/// Convert this port into a new motor object. Semantics are identical to
	/// [`Motor::new()`].
	///
	/// # Panics
	/// Check [`Motor::new()`] semantics.
	#[inline]
	pub fn into_motor(self, reversed: bool, gearset: Gearset, units: EncoderUnits) -> Motor {
		unsafe { Motor::new(self, reversed, gearset, units) }
	}

	/// Convert this port into a new motor object. This function is just a
	/// wrapper for `Port::into_motor(false, Default::default(),
	/// Default::default())`.
	///
	/// # Panics
	/// Will panic if this port does not currently have a motor connected.
	#[inline]
	pub fn into_motor_default(self) -> Motor {
		unsafe { Motor::new(self, false, Default::default(), Default::default()) }
	}

	/// Convert this port into a new rotation sensor object. Semantics are
	/// identical to [`RotationSensor::new()`]
	///
	/// # Panics
	/// Check [`RotationSensor::new()`] semantics.
	#[inline]
	pub fn into_rotation_sensor(self, direction: Direction) -> RotationSensor {
		unsafe { RotationSensor::new(self, direction) }
	}

	/// Convert this port into a new inertial sensor object. Semantics are
	/// identical to [`IMU::new()`]
	///
	/// # Panics
	/// Check [`IMU::new()`] semantics.
	#[inline]
	pub fn into_imu(self) -> IMU {
		unsafe { IMU::new(self) }
	}
}

/// What the type of a device is known to be on a V5 port.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
	/// No Device connected/registered
	None,
	/// V5 Motor
	Motor,
	/// V5 Rotation Sensor,
	Rotation,
	/// V5 Inertial Sensor,
	IMU,
	/// V5 Distance Sensor,
	Distance,
	/// V5 Robot Radio,
	Radio,
	/// V5 Vision Sensor,
	Vision,
	/// V5 3-Wire Expander,
	Adi,
	/// V5 Optical Sensor,
	Optical,
	/// V5 GPS Sensor,
	GPS,
	/// V5 generic serial port,
	Serial,
	/// Undefined device type,
	Undefined,
	/// An Unrecognised type from PROS
	Unknown(u32),
}

impl From<v5_device_e_t> for DeviceType {
	fn from(x: v5_device_e_t) -> Self {
		#[allow(non_upper_case_globals)]
		match x {
			v5_device_e_E_DEVICE_NONE => Self::None,
			v5_device_e_E_DEVICE_MOTOR => Self::Motor,
			v5_device_e_E_DEVICE_ROTATION => Self::Rotation,
			v5_device_e_E_DEVICE_IMU => Self::IMU,
			v5_device_e_E_DEVICE_DISTANCE => Self::Distance,
			v5_device_e_E_DEVICE_RADIO => Self::Radio,
			v5_device_e_E_DEVICE_VISION => Self::Vision,
			v5_device_e_E_DEVICE_ADI => Self::Adi,
			v5_device_e_E_DEVICE_OPTICAL => Self::Optical,
			v5_device_e_E_DEVICE_GPS => Self::GPS,
			v5_device_e_E_DEVICE_GENERIC => Self::Serial,
			v5_device_e_E_DEVICE_UNDEFINED => Self::Undefined,
			x => Self::Unknown(x),
		}
	}
}
