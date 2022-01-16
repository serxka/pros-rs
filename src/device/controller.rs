use smallvec::SmallVec;

use crate::bindings::*;
use crate::device::GenericError;
use crate::util::{to_cstring, PROS_ERR};

use alloc::string::String;

pub struct Controller {
	id: u32,
}

pub const fn master() -> Controller {
	Controller {
		id: controller_id_e_t_E_CONTROLLER_MASTER,
	}
}

pub const fn slave() -> Controller {
	Controller {
		id: controller_id_e_t_E_CONTROLLER_PARTNER,
	}
}

/// Which axis to read when calling [`Controller::get_analog`]
#[repr(u32)]
#[derive(Debug)]
pub enum Axis {
	LeftX = controller_analog_e_t_E_CONTROLLER_ANALOG_LEFT_X,
	LeftY = controller_analog_e_t_E_CONTROLLER_ANALOG_LEFT_Y,
	RightX = controller_analog_e_t_E_CONTROLLER_ANALOG_RIGHT_X,
	RightY = controller_analog_e_t_E_CONTROLLER_ANALOG_RIGHT_Y,
}

/// Which button to read when calling [`Controller::get_button`]
#[repr(u32)]
#[derive(Debug)]
pub enum Button {
	L1 = controller_digital_e_t_E_CONTROLLER_DIGITAL_L1,
	L2 = controller_digital_e_t_E_CONTROLLER_DIGITAL_L2,
	R1 = controller_digital_e_t_E_CONTROLLER_DIGITAL_R1,
	R2 = controller_digital_e_t_E_CONTROLLER_DIGITAL_R2,
	Up = controller_digital_e_t_E_CONTROLLER_DIGITAL_UP,
	Down = controller_digital_e_t_E_CONTROLLER_DIGITAL_DOWN,
	Left = controller_digital_e_t_E_CONTROLLER_DIGITAL_LEFT,
	Right = controller_digital_e_t_E_CONTROLLER_DIGITAL_RIGHT,
	X = controller_digital_e_t_E_CONTROLLER_DIGITAL_X,
	B = controller_digital_e_t_E_CONTROLLER_DIGITAL_B,
	Y = controller_digital_e_t_E_CONTROLLER_DIGITAL_Y,
	A = controller_digital_e_t_E_CONTROLLER_DIGITAL_A,
}

impl Controller {
	pub fn get_analog(&self, axis: Axis) -> Result<i32, GenericError> {
		unsafe {
			match controller_get_analog(self.id, axis as u32) {
				PROS_ERR => Err(GenericError::errno()),
				v => Ok(v),
			}
		}
	}

	pub fn get_button(&self, button: Button) -> Result<bool, GenericError> {
		match unsafe { controller_get_digital(self.id, button as u32) } {
			PROS_ERR => Err(GenericError::errno()),
			1 => Ok(true),
			0 => Ok(false),
			_ => unreachable!(),
		}
	}

	pub fn battery_capacity(&self) -> Result<i32, GenericError> {
		unsafe {
			match controller_get_battery_capacity(self.id) {
				PROS_ERR => Err(GenericError::errno()),
				n => Ok(n),
			}
		}
	}

	pub fn battery_level(&self) -> Result<i32, GenericError> {
		unsafe {
			match controller_get_battery_level(self.id) {
				PROS_ERR => Err(GenericError::errno()),
				n => Ok(n),
			}
		}
	}

	pub fn is_connected(&self) -> Result<bool, GenericError> {
		unsafe {
			match controller_is_connected(self.id) {
				0 => Ok(false),
				1 => Ok(true),
				PROS_ERR => Err(GenericError::errno()),
				_ => unreachable!(),
			}
		}
	}

	pub fn set_text(&self, line: u8, column: u8, text: &str) {
		let cstring = to_cstring(String::from(text));
		unsafe {
			controller_set_text(self.id, line, column, cstring.as_ptr());
		}
	}

	pub fn clear(&self) {
		unsafe {
			controller_clear(self.id);
		}
	}

	pub fn clear_line(&self, line: u8) {
		unsafe {
			controller_clear_line(self.id, line);
		}
	}

	pub fn rumble(&self, pattern: &str) {
		assert!(pattern.len() <= 8);
		let mut cstr: SmallVec<[u8; 9]> = smallvec![0; 9];
		for c in pattern.chars() {
			match c {
				'.' | '-' | ' ' => assert!(
					false,
					"rumble pattern contained invalid characters"
				),
				_ => {}
			}
			cstr.push(c as u8);
		}

		unsafe {
			controller_rumble(self.id, cstr.as_ptr());
		}
	}
}

#[allow(non_snake_case)]
pub mod Battery {
	use crate::bindings::*;
	use crate::device::GenericError;
	use crate::util::{PROS_ERR, PROS_ERR_F};

	pub fn get_capacity() -> Result<f64, GenericError> {
		let r = unsafe { battery_get_capacity() };
		if r == PROS_ERR_F {
			Err(GenericError::errno())
		} else {
			Ok(r)
		}
	}

	pub fn get_current() -> Result<i32, GenericError> {
		unsafe {
			match battery_get_current() {
				PROS_ERR => Err(GenericError::errno()),
				n => Ok(n),
			}
		}
	}

	pub fn get_temperature() -> Result<f64, GenericError> {
		let r = unsafe { battery_get_temperature() };
		if r == PROS_ERR_F {
			Err(GenericError::errno())
		} else {
			Ok(r)
		}
	}

	pub fn get_voltage() -> Result<i32, GenericError> {
		unsafe {
			match battery_get_voltage() {
				PROS_ERR => Err(GenericError::errno()),
				n => Ok(n),
			}
		}
	}
}

#[allow(non_snake_case)]
pub mod Competition {
	use bitflags::bitflags;

	use crate::bindings::*;

	bitflags! {
		/// Bitflags for defining the state of the robot in competition mode
		pub struct CompetitionMode: u8 {
			/// The brain is get in autonomous mode
			const AUTONOMOUS = 0x1 << 0x0;
			/// The brain is disabled
			const DISABLED = 0x1 << 0x1;
			/// The brain is connected to the competition control
			const CONNECTED = 0x1 << 0x2;
		}
	}

	pub fn get_status() -> CompetitionMode {
		let flags = unsafe { competition_get_status() };
		CompetitionMode::from_bits_truncate(flags)
	}

	pub fn is_autonomous() -> bool {
		get_status().contains(CompetitionMode::AUTONOMOUS)
	}

	pub fn is_disabled() -> bool {
		get_status().contains(CompetitionMode::DISABLED)
	}

	pub fn is_connected() -> bool {
		get_status().contains(CompetitionMode::CONNECTED)
	}
}
