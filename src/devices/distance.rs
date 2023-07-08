use crate::bindings::*;

use crate::prelude::Port;

use crate::prelude::DeviceError;

#[derive(Debug)]
pub struct DistanceSensor {
	pub port: Port,
}

impl DistanceSensor {
	pub unsafe fn new(port: Port) -> Result<Self, DeviceError> {
		let distance_sensor = Self { port };
		Ok(distance_sensor)
	}

	#[inline]
	pub fn get_port(&self) -> u8 {
		self.port.get()
	}

	pub fn get_distance(&self) -> Result<i32, DeviceError> {
		pros_unsafe_err!(
			distance_get,
			err = DeviceError::errno_distance(),
			self.get_port()
		)
	}

	pub fn get_confidence(&self) -> Result<i32, DeviceError> {
		pros_unsafe_err!(
			distance_get_confidence,
			err = DeviceError::errno_distance(),
			self.get_port()
		)
	}

	pub fn get_object_size(&self) -> Result<i32, DeviceError> {
		pros_unsafe_err!(
			distance_get_object_size,
			err = DeviceError::errno_distance(),
			self.get_port()
		)
	}
}
