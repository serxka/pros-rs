use crate::bindings::*;
use crate::devices::DeviceError;
use crate::ports::Port;
use crate::util::{PROS_ERR_F, PROS_ERR_U32};

use mint::{Quaternion, Vector3};

/// A struct which holds and presents a connected Inertial measurement unit
/// connected to the V5 Brain.
#[derive(Debug)]
pub struct IMU {
	pub port: Port,
}

impl IMU {
	/// Create a new IMU sensor object and begins calibration. Note that you
	/// will have to obey the semantics of [`IMU::calibrate()`] until you have
	/// verified if it safe to read/write from this sensor.
	///
	/// # Panics
	/// This function will panic if an inertial sensor is not connected on this
	/// port.
	///
	/// # Safety
	/// There must only ever be a single reference to this sensor. It is up to
	/// the caller to make sure there does not exists another device object with
	/// the same port. If there is another device object with the same port this
	/// will result in undefined behaviour and/or panics.
	pub unsafe fn new(port: Port) -> Result<Self, DeviceError> {
		let mut imu = IMU { port };
		imu.calibrate()?;
		Ok(imu)
	}

	#[inline]
	pub fn get_port(&self) -> u8 {
		self.port.get()
	}

	/// This function will start to calibrate the IMU, it is non-blocking and
	/// will immediately exit after calling. Calibration takes about 2 seconds
	/// while all the samples are being collected. After calling this you should
	/// wait until [`IMU::is_calibrating()`] returns false before using any
	/// other IMU functions.
	pub fn calibrate(&mut self) -> Result<(), DeviceError> {
		pros_unsafe_err!(imu_reset, err = DeviceError::errno_imu(), self.get_port())?;
		Ok(())
	}

	/// Get a processed value for the rotation of the IMU sensor as a
	/// quaternion.
	pub fn get_quaternion(&self) -> Result<Quaternion<f64>, DeviceError> {
		let res = unsafe { imu_get_quaternion(self.get_port()) };
		if res.x == PROS_ERR_F && res.y == PROS_ERR_F && res.z == PROS_ERR_F && res.w == PROS_ERR_F
		{
			Err(DeviceError::errno_imu())
		} else {
			Ok(imu_quat_to_quat(res))
		}
	}

	/// Get a processed value for the rotation of the IMU sensor as
	/// total rotations around the Z axis in degrees
	pub fn get_rotation(&self) -> Result<f64, DeviceError> {
		let rotation = unsafe { imu_get_rotation(self.get_port()) };
		if rotation == PROS_ERR_F {
			Err(DeviceError::errno_imu())
		} else {
			Ok(rotation)
		}
	}

	/// Read the raw values from the gryoscope. This is the rate at which it is
	/// turning.
	pub fn get_gyro_rate(&self) -> Result<Vector3<f64>, DeviceError> {
		let res = unsafe { imu_get_gyro_rate(self.get_port()) };
		if res.x == PROS_ERR_F && res.y == PROS_ERR_F && res.z == PROS_ERR_F {
			Err(DeviceError::errno_imu())
		} else {
			Ok(imu_raw_into_vec3(res))
		}
	}

	/// Read all three of the raw values for IMU sensors accelerometer axes.
	pub fn get_acceleration(&self) -> Result<Vector3<f64>, DeviceError> {
		let res = unsafe { imu_get_accel(self.get_port()) };
		if res.x == PROS_ERR_F && res.y == PROS_ERR_F && res.z == PROS_ERR_F {
			Err(DeviceError::errno_imu())
		} else {
			Ok(imu_raw_into_vec3(res))
		}
	}

	/// Reset the current rotation of the IMU sensor to 0. This will zero the
	/// rotation quaternion stored internally.
	pub fn tare(&self) -> Result<(), DeviceError> {
		pros_unsafe_err!(imu_tare, err = DeviceError::errno_imu(), self.get_port())?;
		Ok(())
	}

	/// Check to see if the IMU sensor is currently calibrating.
	pub fn is_calibrating(&self) -> Result<bool, DeviceError> {
		match unsafe { imu_get_status(self.get_port()) } {
			PROS_ERR_U32 => Err(DeviceError::errno_imu()),
			s if s == imu_status_e_E_IMU_STATUS_ERROR => Err(DeviceError::Unknown),
			// We know for sure that it is calibrating
			s if s & imu_status_e_E_IMU_STATUS_CALIBRATING != 0 => Ok(true),
			// We probably got 0 meaning it's calibrated, otherwise rubbish
			// value and we are screwed anyway
			_ => Ok(false),
		}
	}
}

fn imu_raw_into_vec3(f: imu_raw_s) -> Vector3<f64> {
	[f.x, f.y, f.z].into()
}

fn imu_quat_to_quat(f: quaternion_s) -> Quaternion<f64> {
	Quaternion {
		v: [f.x, f.y, f.z].into(),
		s: f.w,
	}
}
