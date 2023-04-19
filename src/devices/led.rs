use crate::bindings::*;
use crate::devices::DeviceError;
use crate::ports::TriPort;
use crate::util::Colour;

#[derive(Debug)]
pub struct LedStrip {
	_triport: TriPort,
	led: ext_adi_led_t,
	colours: [u32; Self::MAX_LED],
}

impl LedStrip {
	/// The maximum amount of LEDs addressable on a single port.
	pub const MAX_LED: usize = 64;

	pub unsafe fn new(triport: TriPort) -> Result<Self, DeviceError> {
		let led = pros_unsafe_err!(
			ext_adi_led_init,
			err = DeviceError::errno_adi(),
			triport.ext_port.get(),
			triport.port.get()
		)?;
		let colours = [Colour::WHITE.as_u32(); Self::MAX_LED];
		Ok(LedStrip {
			_triport: triport,
			led,
			colours,
		})
	}

	pub fn set_all(&mut self, colour: Colour) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			ext_adi_led_set_all,
			err = DeviceError::errno_adi(),
			self.led,
			self.colours.as_mut_ptr(),
			self.colours.len() as u32,
			colour.as_u32()
		)?;
		Ok(())
	}

	pub fn clear_all(&mut self) -> Result<(), DeviceError> {
		self.set_all(Colour::WHITE)
	}

	pub fn set_pixel(&mut self, colour: Colour, index: u32) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			ext_adi_led_set_pixel,
			err = DeviceError::errno_adi(),
			self.led,
			self.colours.as_mut_ptr(),
			self.colours.len() as u32,
			colour.as_u32(),
			index
		)?;
		Ok(())
	}

	pub fn clear_pixel(&mut self, index: u32) -> Result<(), DeviceError> {
		self.set_pixel(Colour::WHITE, index)
	}
}
