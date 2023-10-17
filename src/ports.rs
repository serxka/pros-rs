//! Contains types for managing the ports on the V5 Brain.

use crate::bindings::*;
use crate::devices::{
	distance::*, gps::*, imu::*, led::*, motor::*, rotation::*, vision::*, DeviceError, Direction,
};

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
	pub unsafe fn new(port: u8) -> Option<Port> {
		if (1..=21).contains(&port) {
			Some(Port(NonZeroU8::new_unchecked(port)))
		} else {
			None
		}
	}

	/// Create a new port without checking the range at all. The purpose of this
	/// function is to be able to create references to internal ports.
	///
	/// # Safety
	/// The user must make sure this value is not 0, and it is a valid port on
	/// the V5.
	pub unsafe fn new_unchecked(port: u8) -> Self {
		Port(NonZeroU8::new_unchecked(port))
	}

	pub(crate) unsafe fn clone(&self) -> Self {
		Port(self.0)
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
	pub fn into_motor(
		self,
		reversed: bool,
		gearset: Gearset,
		units: EncoderUnits,
	) -> Result<Motor, DeviceError> {
		unsafe { Motor::new(self, reversed, gearset, units) }
	}

	/// Convert this port into a new motor object. This function is just a
	/// wrapper for `Port::into_motor(false, Default::default(),
	/// Default::default())`.
	///
	/// # Panics
	/// Will panic if this port does not currently have a motor connected.
	#[inline]
	pub fn into_motor_default(self) -> Result<Motor, DeviceError> {
		unsafe { Motor::new(self, false, Default::default(), Default::default()) }
	}

	/// Convert this port into a new rotation sensor object. Semantics are
	/// identical to [`RotationSensor::new()`]
	///
	/// # Panics
	/// Check [`RotationSensor::new()`] semantics.
	#[inline]
	pub fn into_rotation_sensor(self, direction: Direction) -> Result<RotationSensor, DeviceError> {
		unsafe { RotationSensor::new(self, direction) }
	}

	/// Convert this port into a new inertial sensor object. Semantics are
	/// identical to [`IMU::new()`]
	///
	/// # Panics
	/// Check [`IMU::new()`] semantics.
	#[inline]
	pub fn into_imu(self) -> Result<IMU, DeviceError> {
		unsafe { IMU::new(self) }
	}

	/// Convert this port into a new game positioning system object. Semantics
	/// are identical to [`GPS::new()`]
	///
	/// # Panics
	/// Check [`GPS::new()`] semantics.
	#[inline]
	pub fn into_gps(self) -> Result<GPS, DeviceError> {
		unsafe { GPS::new(self) }
	}

	/// Convert this port into a new vision sensor object. Semantics are
	/// identical to [`Vision::new()`]
	///
	/// # Errors
	/// Check [`Vision::new()`] semantics.
	#[inline]
	pub fn into_vision(self) -> Result<Vision, DeviceError> {
		unsafe { Vision::new(self) }
	}

	/// Convert this port into a new distance sensor object. Semantics are
	/// identical to [`DistanceSensor::new()`]
	///
	/// # Errors
	/// Check [`DistanceSensor::new()`] semantics.
	#[inline]
	pub fn into_distance(self) -> Result<DistanceSensor, DeviceError> {
		unsafe { DistanceSensor::new(self) }
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

/// An object of a TriPort on the V5 Brain.
///
/// This is an object which should not be created by hand in most cases. It uses
/// ownership semantics at runtime to help make sure a port is not used for more
/// than one device. It also uses types to make a port is isn't use for more
/// than one purpose.
#[derive(Debug)]
pub struct TriPort {
	pub(crate) port: NonZeroU8,
	pub(crate) ext_port: Port,
}

impl TriPort {
	/// Creates a new TriPort object, this object should then be converted into
	/// the specific type which it will be used for.
	///
	/// If `ext_port` is `None` then the internal TriPorts will be used,
	/// otherwise an TriPort expander will be used as the interface.
	///
	/// # Assertions
	/// Asserts that the new TriPort is within a valid range.
	///
	/// # Safety
	/// The users must make sure there is not more than one TriPort object
	/// created for a certain port. The user must also make sure that `ext_port`
	/// is a device of [`DeviceType::Adi`].
	pub unsafe fn new(port: u8, ext_port: Option<Port>) -> Option<Self> {
		if (1..=8).contains(&port) {
			Some(TriPort {
				port: NonZeroU8::new_unchecked(port),
				// Port 22 is the internal ADI expander port
				ext_port: ext_port.unwrap_or(Port::new_unchecked(22)),
			})
		} else {
			None
		}
	}

	pub unsafe fn new_unchecked(port: u8, ext_port: Option<Port>) -> Self {
		TriPort {
			port: NonZeroU8::new_unchecked(port),
			// Port 22 is the internal ADI expander port
			ext_port: ext_port.unwrap_or(Port::new_unchecked(22)),
		}
	}

	/// Used internally to set the mode of a port
	pub(crate) unsafe fn set_mode(&mut self, mode: TriPortMode) -> Result<(), DeviceError> {
		match ext_adi_port_set_config(self.ext_port.get(), self.port.get(), mode.into()) {
			crate::util::PROS_ERR => Err(DeviceError::errno_generic()),
			x => Ok(x),
		}?;
		Ok(())
	}

	#[inline]
	pub fn get(&self) -> (u8, u8) {
		(self.port.get(), self.ext_port.get())
	}

	pub fn into_led_strip(self) -> Result<LedStrip, DeviceError> {
		unsafe { LedStrip::new(self) }
	}
}

pub(crate) enum TriPortMode {
	AnalogIn,
	AnalogOut,
	DigitalIn,
	DigitalOut,
}

impl From<TriPortMode> for adi_port_config_e_t {
	fn from(m: TriPortMode) -> Self {
		match m {
			TriPortMode::AnalogIn => adi_port_config_e_E_ADI_ANALOG_IN,
			TriPortMode::AnalogOut => adi_port_config_e_E_ADI_ANALOG_OUT,
			TriPortMode::DigitalIn => adi_port_config_e_E_ADI_DIGITAL_IN,
			TriPortMode::DigitalOut => adi_port_config_e_E_ADI_DIGITAL_OUT,
		}
	}
}

/// A helper trait for converting TriPorts into different modes.
pub trait TriPortConvert {
	/// Converts this TriPort object into a
	/// [`TriPortAnalogIn`][modes::TriPortAnalogIn].
	///
	/// # Panics
	/// This function will panic if the TriPort is no longer connected.
	fn into_analog_in(self) -> Result<modes::TriPortAnalogIn, DeviceError>;

	/// Converts this TriPort object into a
	/// [`TriPortAnalogOut`][modes::TriPortAnalogOut].
	///
	/// # Panics
	/// This function will panic if the TriPort is no longer connected.
	fn into_analog_out(self) -> Result<modes::TriPortAnalogOut, DeviceError>;

	/// Converts this TriPort object into a
	/// [`TriPortDigitalIn`][modes::TriPortDigitalIn].
	///
	/// # Panics
	/// This function will panic if the TriPort is no longer connected.
	fn into_digital_in(self) -> Result<modes::TriPortDigitalIn, DeviceError>;

	/// Converts this TriPort object into a
	/// [`TriPortDigitalOut`][modes::TriPortDigitalOut].
	///
	/// # Panics
	/// This function will panic if the TriPort is no longer connected.
	fn into_digital_out(self) -> Result<modes::TriPortDigitalOut, DeviceError>;
}

pub mod modes {
	//! Modes of operations for the TriPorts.

	use super::{TriPort, TriPortConvert, TriPortMode};
	use crate::bindings::*;
	use crate::devices::DeviceError;

	impl TriPortConvert for TriPort {
		fn into_analog_in(mut self) -> Result<TriPortAnalogIn, DeviceError> {
			unsafe {
				self.set_mode(TriPortMode::AnalogIn)?;
			}
			Ok(TriPortAnalogIn(self))
		}
		fn into_analog_out(mut self) -> Result<TriPortAnalogOut, DeviceError> {
			unsafe { self.set_mode(TriPortMode::AnalogOut)? }
			Ok(TriPortAnalogOut(self))
		}
		fn into_digital_in(mut self) -> Result<TriPortDigitalIn, DeviceError> {
			unsafe { self.set_mode(TriPortMode::DigitalIn)? }
			Ok(TriPortDigitalIn(self))
		}
		fn into_digital_out(mut self) -> Result<TriPortDigitalOut, DeviceError> {
			unsafe { self.set_mode(TriPortMode::DigitalOut)? }
			Ok(TriPortDigitalOut(self))
		}
	}

	// [`ext_adi_port_get_value()`] and [`ext_adi_port_set_value()`] cannot
	// fail, as such the function doesn't return an error. Since the only
	// possible error for writing is that the port is it's not connected, we
	// don't care about those errors as there is no guarantee the signal will
	// actually arrive.

	/// Wrapping of a TriPort, limiting it to being a single analog input.
	pub struct TriPortAnalogIn(TriPort);
	impl TriPortAnalogIn {
		/// Read an analog value from the TriPort. The TriPort has a 12-bit ADC
		/// which means this value will be between 0 - 4095.
		pub fn read(&self) -> i32 {
			unsafe { ext_adi_port_get_value(self.0.ext_port.get(), self.0.port.get()) }
		}
	}

	/// Wrapping of a TriPort, limiting it to being a single analog output.
	pub struct TriPortAnalogOut(TriPort);
	impl TriPortAnalogOut {
		/// Write an analog value to the TriPort. Valid analog values are
		/// between 0 - 4095.
		pub fn write(&mut self, value: u16) {
			unsafe {
				ext_adi_port_set_value(self.0.ext_port.get(), self.0.port.get(), value as i32);
			}
		}
	}

	/// Wrapping of a TriPort, limiting it to being a single digital input.
	pub struct TriPortDigitalIn(TriPort);
	impl TriPortDigitalIn {
		/// Read a digital value from the TriPort. This value is either `HIGH`
		/// which is represented by a `true` value or `LOW` which is represented
		/// by a `false`.
		pub fn read(&self) -> bool {
			let res = unsafe { ext_adi_port_get_value(self.0.ext_port.get(), self.0.port.get()) };
			!(res == crate::util::PROS_ERR || res == 0)
		}
	}

	/// Wrapping of a TriPort, limiting it to being a single digital output.
	pub struct TriPortDigitalOut(TriPort);
	impl TriPortDigitalOut {
		/// Write either `HIGH` or `LOW` to the TriPort, represented by `true`
		/// and `false` respectively.
		pub fn write(&mut self, value: bool) {
			unsafe {
				ext_adi_port_set_value(self.0.ext_port.get(), self.0.port.get(), value as i32);
			}
		}
	}

	macro_rules! impl_triport_convert {
		($tri:tt) => {
			impl TriPortConvert for $tri {
				fn into_analog_in(self) -> Result<TriPortAnalogIn, DeviceError> {
					self.0.into_analog_in()
				}
				fn into_analog_out(self) -> Result<TriPortAnalogOut, DeviceError> {
					self.0.into_analog_out()
				}
				fn into_digital_in(self) -> Result<TriPortDigitalIn, DeviceError> {
					self.0.into_digital_in()
				}
				fn into_digital_out(self) -> Result<TriPortDigitalOut, DeviceError> {
					self.0.into_digital_out()
				}
			}
		};
	}

	impl_triport_convert!(TriPortAnalogIn);
	impl_triport_convert!(TriPortAnalogOut);
	impl_triport_convert!(TriPortDigitalIn);
	impl_triport_convert!(TriPortDigitalOut);
}
