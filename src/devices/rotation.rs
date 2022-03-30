use crate::bindings::*;
use crate::devices::{DeviceError, Direction};
use crate::ports::Port;

/// A struct which holds and presents a connected rotation sensor connected to
/// the V5 Brain.
#[derive(Debug)]
pub struct RotationSensor {
	pub port: Port,
}

impl RotationSensor {
	/// Create a new rotation sensor with the specified port and the specified
	/// forwards clockwise direction.
	///
	/// # Panics
	/// This function will panic if a rotation sensor is not connected on this
	/// port.
	///
	/// # Safety
	/// There must only ever be a single reference to this sensor. It is up to
	/// the caller to make sure there does not exists another device object with
	/// the same port. If there is another device object with the same port this
	/// will result in undefined behaviour and/or panics.
	pub unsafe fn new(port: Port, direction: Direction) -> Self {
		let mut s = RotationSensor { port };
		s.set_direction(direction).unwrap();
		s
	}

	#[inline]
	fn get_port(&self) -> u8 {
		self.port.get()
	}

	/// Resets the rotations sensor absolute value to be the same as the current
	/// rotation sensor's angle. i.e. `absolue_ticks = absolue_ticks %
	/// tick_per_rotation`.
	pub fn reset(&mut self) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			rotation_reset_position,
			err = DeviceError::errno_rotation(),
			self.get_port()
		)?;
		Ok(())
	}

	/// Get the rotation sensor absolute rotation value in centidegrees.
	pub fn get_position(&self) -> Result<i32, DeviceError> {
		pros_unsafe_err!(
			rotation_get_position,
			err = DeviceError::errno_rotation(),
			self.get_port()
		)
	}

	/// Get the rotation sensor's current velocity in centidegrees per second.
	pub fn get_velocity(&self) -> Result<i32, DeviceError> {
		pros_unsafe_err!(
			rotation_get_velocity,
			err = DeviceError::errno_rotation(),
			self.get_port()
		)
	}

	/// Get the rotation sensor's current angle in centigrees, a value between 0
	/// and 36000.
	pub fn get_angle(&self) -> Result<i32, DeviceError> {
		pros_unsafe_err!(
			rotation_get_angle,
			err = DeviceError::errno_rotation(),
			self.get_port()
		)
	}

	/// This will update the current direction in the rotation sensor to be
	/// considered as the forwards direction. This will not reverse the
	/// currently stored value in the sensor.
	pub fn set_direction(&mut self, direction: Direction) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			rotation_set_reversed,
			err = DeviceError::errno_rotation(),
			self.get_port(),
			direction == Direction::Reverse
		)?;
		Ok(())
	}

	/// Check which direction is currently considered as forward.
	pub fn get_direction(&self) -> Result<Direction, DeviceError> {
		let rev = pros_unsafe_err!(
			rotation_get_reversed,
			err = DeviceError::errno_rotation(),
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
	///
	/// # Assertions
	/// This function will assert that rate is a value greater than `5`.
	pub fn set_data_rate(&mut self, rate: u32) -> Result<(), DeviceError> {
		assert!(rate >= 5);
		pros_unsafe_err!(
			rotation_set_data_rate,
			err = DeviceError::errno_rotation(),
			self.get_port(),
			rate
		)?;
		Ok(())
	}
}
