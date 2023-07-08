use crate::bindings::*;
use crate::devices::DeviceError;
use crate::ports::Port;
use crate::util::PROS_ERR_F;

use crate::math::vec::{Vec2, Vec3};

/// A struct which holds and presents a connected Game Positioning System
/// connected to the V5 Brain.
pub struct GPS {
	pub port: Port,
}

#[derive(Debug, Copy, Clone)]
pub struct State {
	pub position: Vec2,
	pub pitch: f64,
	pub roll: f64,
	pub yaw: f64,
}

impl GPS {
	/// Create a new GPS sensor object. This will not call any SDK calls to the
	/// GPS sensor, after obtaining a handle to this object it is the
	/// responsibility of the user to call functions to set it up correctly. The
	/// optimal way to do this is by calling [`GPS::initialise()`].
	///
	/// # Safety
	/// There must only ever be a single reference to this sensor. It is up to
	/// the caller to make sure there does not exists another device object with
	/// the same port. If there is another device object with the same port this
	/// will result in undefined behaviour and/or panics.
	pub unsafe fn new(port: Port) -> Result<Self, DeviceError> {
		let gps = GPS { port };
		Ok(gps)
	}

	#[inline]
	pub fn get_port(&self) -> u8 {
		self.port.get()
	}

	/// This function is the equivalent of calling both [`GPS::set_offset`] and
	/// [`GPS::set_position`]. Refer to the those functions for how the
	/// parameters are interpreted.
	pub fn initialise(
		&mut self,
		offset: Vec2,
		initial: Vec2,
		heading: f64,
	) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			gps_initialize_full,
			err = DeviceError::errno_imu(),
			self.get_port(),
			offset.x,
			offset.y,
			initial.x,
			initial.y,
			heading
		)?;
		Ok(())
	}

	/// Set the GPS's sensor location offset relative to the centre of the
	/// robot's turning point in meters.
	pub fn set_offset(&mut self, offset: Vec2) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			gps_set_offset,
			err = DeviceError::errno_imu(),
			self.get_port(),
			offset.x,
			offset.y
		)?;
		Ok(())
	}

	/// Set the robot's location relative to the centre of the field in meters.
	/// Position is the offset from centre of the field which is marked at (0,
	/// 0). The heading of the robot is also set in degrees.
	pub fn set_position(&mut self, position: Vec2, heading: f64) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			gps_set_position,
			err = DeviceError::errno_imu(),
			self.get_port(),
			position.x,
			position.y,
			heading
		)?;
		Ok(())
	}

	/// Set the rotation of the GPS sensor in degrees.
	pub fn set_rotation(&mut self, target: f64) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			gps_set_rotation,
			err = DeviceError::errno_imu(),
			self.get_port(),
			target
		)?;
		Ok(())
	}

	/// Set the GPS sensor's data rate in milliseconds, this is only applicable
	/// to the IMU onboard the GPS. The data rate will be rounded down to the
	/// closest multiple of 5 ms with a minimum possible rate of 5 ms.
	pub fn set_data_rate(&mut self, rate: u32) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			gps_set_data_rate,
			err = DeviceError::errno_imu(),
			self.get_port(),
			rate
		)?;
		Ok(())
	}

	/// Get the possible RMS (Root Mean Squared) error in meters for the GPS
	/// position.
	pub fn get_error(&self) -> Result<f64, DeviceError> {
		pros_unsafe_err_f!(
			gps_get_error,
			err = DeviceError::errno_imu(),
			self.get_port()
		)
	}

	/// Get the state of the GPS sensor, this will return the position on the
	/// field along with the roll, yaw and pitch.
	pub fn get_state(&self) -> Result<State, DeviceError> {
		let res = unsafe { gps_get_status(self.get_port()) };
		if res.x == PROS_ERR_F
			&& res.y == PROS_ERR_F
			&& res.pitch == PROS_ERR_F
			&& res.roll == PROS_ERR_F
			&& res.yaw == PROS_ERR_F
		{
			Err(DeviceError::errno_imu())
		} else {
			Ok(res.into())
		}
	}

	/// Get the heading of the GPS sensor. This will return a value within
	/// [0, 360) degrees.
	pub fn get_heading(&self) -> Result<f64, DeviceError> {
		pros_unsafe_err_f!(
			gps_get_heading,
			err = DeviceError::errno_imu(),
			self.get_port()
		)
	}

	/// Get the elapsed rotation of the GPS sensor in degrees. This will count
	/// up above 360 degrees or below 0 degrees.
	pub fn get_rotation(&self) -> Result<f64, DeviceError> {
		pros_unsafe_err_f!(
			gps_get_rotation,
			err = DeviceError::errno_imu(),
			self.get_port()
		)
	}

	/// This will tare the current rotation to zero degrees. Until the GPS
	/// registers movement `[GPS::get_heading()]` will report 0 degrees.
	pub fn tare_rotation(&mut self) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			gps_tare_rotation,
			err = DeviceError::errno_imu(),
			self.get_port()
		)?;
		Ok(())
	}

	/// Get the rate at which the inbuilt IMU is rotating.
	pub fn get_gyro_rate(&self) -> Result<Vec3, DeviceError> {
		let res = unsafe { gps_get_gyro_rate(self.get_port()) };
		if res.x == PROS_ERR_F && res.y == PROS_ERR_F && res.z == PROS_ERR_F {
			Err(DeviceError::errno_imu())
		} else {
			Ok(Vec3::new(res.x, res.y, res.z))
		}
	}

	/// Get the rate at which the inbuilt IMU is moving.
	pub fn get_acceleration(&self) -> Result<Vec3, DeviceError> {
		let res = unsafe { gps_get_accel(self.get_port()) };
		if res.x == PROS_ERR_F && res.y == PROS_ERR_F && res.z == PROS_ERR_F {
			Err(DeviceError::errno_imu())
		} else {
			Ok(Vec3::new(res.x, res.y, res.z))
		}
	}
}

impl From<gps_status_s> for State {
	fn from(f: gps_status_s) -> State {
		State {
			position: Vec2::new(f.x, f.y),
			pitch: f.pitch,
			roll: f.roll,
			yaw: f.yaw,
		}
	}
}
