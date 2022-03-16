use crate::bindings::*;
use crate::util::{get_errno, Port, PROS_ERR, PROS_ERR_F, PROS_ERR_U32};

/// Possible errors that could be returned from IMU function calls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IMUError {
	/// The port chosen cannot be configured as an IMU
	PortNotIMU,
	/// The port chosen is not within the range of supported ports of the
	/// V5 Brain
	PortRange,
	/// The sensor is currently calibrating
	StillCalibrating,
	/// An unknown error
	#[doc(hidden)]
	Unknown(i32),
}

impl IMUError {
	pub(crate) fn errno() -> Self {
		match get_errno() {
			libc::ENODEV => Self::PortNotIMU,
			libc::ENXIO => Self::PortRange,
			libc::EAGAIN => Self::StillCalibrating,
			e => Self::Unknown(e),
		}
	}
}

/// A three dimensional vector for storing any types of values.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

/// A quaterion for storing rotations
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub w: f64,
}

/// A struct which holds and presents a connected Inertial measurement unit
/// connected to the V5 brain
#[derive(Debug)]
pub struct IMU {
	pub port: Port,
}

impl IMU {
	/// Create a new IMU sensor object and begins calibration. Note that you
	/// will have to obey the semantics of [`IMU::calibrate()`] until you have
	/// verified if it safe to read/write from this sensor.
	pub fn new(port: Port) -> Result<Self, IMUError> {
		let mut imu = IMU { port };
		imu.calibrate()?;
		Ok(imu)
	}

	fn get_port(&self) -> u8 {
		self.port.get()
	}

	/// This function will start to calibrate the IMU, it is non-blocking and
	/// will immediately exit after calling. Calibration takes about 2 seconds
	/// while all the samples are being collected. After calling this you should
	/// wait until [`IMU::is_calibrating()`] returns false before using any
	/// other IMU functions.
	pub fn calibrate(&mut self) -> Result<(), IMUError> {
		pros_unsafe_err!(imu_reset, err = IMUError, self.get_port())?;
		Ok(())
	}

	/// Get a processed value for the rotation of the IMU sensor as a
	/// quaternion.
	pub fn get_quaternion(&self) -> Result<Quaternion, IMUError> {
		let res = unsafe { imu_get_quaternion(self.get_port()) };
		if res.x == PROS_ERR_F
			&& res.y == PROS_ERR_F
			&& res.z == PROS_ERR_F
			&& res.w == PROS_ERR_F
		{
			Err(IMUError::errno())
		} else {
			Ok(Quaternion {
				x: res.x,
				y: res.y,
				z: res.z,
				w: res.w,
			})
		}
	}

	/// Read the raw values from the gryoscope. This is the rate at which it is
	/// turning.
	pub fn get_gyro_rate(&self) -> Result<Vec3, IMUError> {
		let res = unsafe { imu_get_gyro_rate(self.get_port()) };
		if res.x == PROS_ERR_F && res.y == PROS_ERR_F && res.z == PROS_ERR_F {
			Err(IMUError::errno())
		} else {
			Ok(Vec3 {
				x: res.x,
				y: res.y,
				z: res.z,
			})
		}
	}

	/// Read all three of the raw values for IMU sensors accelerometer axes.
	pub fn get_acceleration(&self) -> Result<Vec3, IMUError> {
		let res = unsafe { imu_get_accel(self.get_port()) };
		if res.x == PROS_ERR_F && res.y == PROS_ERR_F && res.z == PROS_ERR_F {
			Err(IMUError::errno())
		} else {
			Ok(Vec3 {
				x: res.x,
				y: res.y,
				z: res.z,
			})
		}
	}

	/// Reset the current rotation of the IMU sensor to 0. This will zero the
	/// rotation quaternion stored internally.
	pub fn tare(&self) -> Result<(), IMUError> {
		pros_unsafe_err!(imu_tare, err = IMUError, self.get_port())?;
		Ok(())
	}

	/// Check to see if the IMU sensor is calibrating
	pub fn is_calibrating(&self) -> Result<bool, IMUError> {
		match unsafe { imu_get_status(self.get_port()) } {
			PROS_ERR_U32 => Err(IMUError::errno()),
			s if s == imu_status_e_E_IMU_STATUS_ERROR => {
				Err(IMUError::Unknown(PROS_ERR))
			}
			// We know for sure that it is calibrating
			s if s & imu_status_e_E_IMU_STATUS_CALIBRATING != 0 => Ok(true),
			// We probably got 0 meaning it's calibrated, otherwise rubbish
			// value and we are screwed anyway
			_ => Ok(false),
		}
	}
}
