use crate::bindings::*;
use crate::util::{get_errno, Port, PROS_ERR, PROS_ERR_U32};

/// Possible errors that could be returned from IMU function calls
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
pub struct Vec3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

pub type RotationRate = Vec3;
pub type Acceleration = Vec3;

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
		unimplemented!()
	}

	pub fn get_gyro_rate(&self) -> Result<RotationRate, IMUError> {
		unimplemented!()
	}

	pub fn get_acceleration(&self) -> Result<Acceleration, IMUError> {
		unimplemented!()
	}

	/// Check to see if the IMU sensor is calibrating
	pub fn is_calibrating(&self) -> Result<bool, IMUError> {
		match unsafe { imu_get_status(self.get_port()) } {
			PROS_ERR_U32 => Err(IMUError::errno()),
			s if s == imu_status_e_E_IMU_STATUS_ERROR => {
				Err(IMUError::Unknown(PROS_ERR))
			}
			// We know for sure that it is calibrating
			s if s == imu_status_e_E_IMU_STATUS_CALIBRATING => Ok(true),
			// We probably got 0 meaning it's calibrated, otherwise rubbish
			// value and we are screwed anyway
			_ => Ok(false),
		}
	}
}
