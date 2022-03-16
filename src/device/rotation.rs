use crate::bindings::*;
use crate::device::Direction;
use crate::util::{get_errno, Port};

/// Possible errors that could be returned from Rotation sensor function calls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationError {
	/// The Port chosen cannot be configured as a rotation sensor
	PortNotRotationSensor,
	/// The Port chosen is not within the range of supported ports of the
	/// V5 Brain
	PortRange,
	/// An unknown error
	#[doc(hidden)]
	Unknown(i32),
}

impl RotationError {
	pub(crate) fn errno() -> Self {
		match get_errno() {
			libc::ENODEV => Self::PortNotRotationSensor,
			libc::ENXIO => Self::PortRange,
			e => Self::Unknown(e),
		}
	}
}

/// A struct which holds and presents a connected rotation sensor connected to
/// the V5 brain
#[derive(Debug)]
pub struct RotationSensor {
	pub port: Port,
}

impl RotationSensor {
	/// Create a new rotation sensor with the specified port and the specified
	/// forwards clockwise direction.
	pub fn new(
		port: Port,
		direction: Direction,
	) -> Result<Self, RotationError> {
		let mut s = RotationSensor { port };
		s.set_direction(direction)?;
		Ok(s)
	}

	fn get_port(&self) -> u8 {
		self.port.get()
	}

	/// Resets the rotations sensor absolute value to be the same as the current
	/// rotation sensor's angle. i.e. `absolue_ticks = absolue_ticks %
	/// tick_per_rotation`.
	pub fn reset(&mut self) -> Result<(), RotationError> {
		pros_unsafe_err!(
			rotation_reset_position,
			err = RotationError,
			self.get_port()
		)?;
		Ok(())
	}

	/// Get the rotation sensor absolute rotation value in centidegrees.
	pub fn get_position(&self) -> Result<i32, RotationError> {
		pros_unsafe_err!(
			rotation_get_position,
			err = RotationError,
			self.get_port()
		)
	}

	/// Get the rotation sensor's current velocity in centidegrees per second.
	pub fn get_velocity(&self) -> Result<i32, RotationError> {
		pros_unsafe_err!(
			rotation_get_velocity,
			err = RotationError,
			self.get_port()
		)
	}

	/// Get the rotation sensor's current angle in centigrees, a value between 0
	/// and 36000.
	pub fn get_angle(&self) -> Result<i32, RotationError> {
		pros_unsafe_err!(
			rotation_get_angle,
			err = RotationError,
			self.get_port()
		)
	}

	/// This will update the current direction in the rotation sensor to be
	/// considered as the forwards direction. This will not reverse the
	/// currently stored value in the sensor.
	pub fn set_direction(
		&mut self,
		direction: Direction,
	) -> Result<(), RotationError> {
		pros_unsafe_err!(
			rotation_set_reversed,
			err = RotationError,
			self.get_port(),
			direction == Direction::Reverse
		)?;
		Ok(())
	}

	/// Check which direction is currently considered as forward.
	pub fn get_direction(&self) -> Result<Direction, RotationError> {
		let rev = pros_unsafe_err!(
			rotation_get_reversed,
			err = RotationError,
			self.get_port()
		)?;
		if rev == 0 {
			Ok(Direction::Forward)
		} else {
			Ok(Direction::Reverse)
		}
	}

	/// Set the rotation sensor's refresh interval in milliseconds. The rate may
	/// be specified as increments of 5ms, if not it will be rounded down to
	/// nearest increment. The smallest allowable refresh rate is 5ms. The
	/// default is 10ms.
	pub fn set_data_rate(&mut self, rate: u32) -> Result<(), RotationError> {
		assert!(rate >= 5);
		pros_unsafe_err!(
			rotation_set_data_rate,
			err = RotationError,
			self.get_port(),
			rate
		)?;
		Ok(())
	}
}
