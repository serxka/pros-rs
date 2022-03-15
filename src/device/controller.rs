use smallvec::SmallVec;

use crate::bindings::*;
use crate::device::GenericError;
use crate::util::to_cstring;

use alloc::string::String;

/// A reference to a certain connected controller
pub struct Controller {
	id: u32,
}

/// Get a reference to the master controller on the V5 Brain
pub const fn master() -> Controller {
	Controller {
		id: controller_id_e_t_E_CONTROLLER_MASTER,
	}
}

/// Get a reference to the slave controller on the V5 Brain
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
	/// Gets the value of an analog axis (joystick) on a controller.
	pub fn get_analog(&self, axis: Axis) -> Result<i8, GenericError> {
		let res = pros_unsafe_err!(
			controller_get_analog,
			err = GenericError,
			self.id,
			axis as u32
		)?;
		Ok(res as i8)
	}

	/// Get the value of a digital axis (button) on a controller. If the axis is
	/// high a `true` boolean is return, likewise if it low a `false` is
	/// returned.
	pub fn get_button(&self, button: Button) -> Result<bool, GenericError> {
		pros_unsafe_err_bool!(
			controller_get_digital,
			err = GenericError,
			self.id,
			button as u32
		)
	}

	/// Gets the battery capacity for the given controller.
	pub fn battery_capacity(&self) -> Result<i32, GenericError> {
		pros_unsafe_err!(
			controller_get_battery_capacity,
			err = GenericError,
			self.id
		)
	}

	/// Gets the battery level for the given controller.
	pub fn battery_level(&self) -> Result<i32, GenericError> {
		pros_unsafe_err!(
			controller_get_battery_level,
			err = GenericError,
			self.id
		)
	}

	/// Tests to see if this controller is currently connected or not.
	pub fn is_connected(&self) -> Result<bool, GenericError> {
		pros_unsafe_err_bool!(
			controller_is_connected,
			err = GenericError,
			self.id
		)
	}

	/// Sets a segments of characters on the controller display to a value. A
	/// line and column for the cursor must also be supplied. Any text that does
	/// not fit onto the screen is truncated and discarded.
	pub fn set_text(&mut self, line: u8, column: u8, text: &str) {
		let cstring = to_cstring(String::from(text));
		unsafe {
			controller_set_text(self.id, line, column, cstring.as_ptr());
		}
	}

	/// Clear the entire character display on the controller.
	pub fn clear(&mut self) {
		unsafe {
			controller_clear(self.id);
		}
	}

	/// Clear a single line of text on the character display on the controller.
	pub fn clear_line(&mut self, line: u8) {
		unsafe {
			controller_clear_line(self.id, line);
		}
	}

	/// Send a rumble pattern to the controller. The pattern can consist of the
	/// characters: '.' = short rumble, '-' = long rumble, ' ' = pause. The
	/// maximum supported length for patterns is 8 characters, any invalid
	/// character will get discarded.
	///
	/// ```rust
	/// controller::master().rumble(b".--..  -");
	/// ```
	pub fn rumble(&mut self, pattern: &[u8]) {
		let mut cstr: SmallVec<[u8; 9]> = smallvec![0; 9];
		for c in pattern {
			// We don't want to read more than 8 bytes of the pattern
			if cstr.len() == 8 {
				break;
			}
			match c {
				b'.' | b'-' | b' ' => cstr.push(*c),
				_ => {
					// We just ignore any invalid characters
				}
			}
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

	/// Get the current capacity of the battery.
	pub fn get_capacity() -> Result<f64, GenericError> {
		pros_unsafe_err_f!(battery_get_capacity, err = GenericError)
	}

	/// Get the amount of current that is currently being drawn from the
	/// battery.
	pub fn get_current() -> Result<i32, GenericError> {
		pros_unsafe_err!(battery_get_current, err = GenericError)
	}

	/// Get the temperature of the battery. This is helpful for supplying any
	/// warnings.
	pub fn get_temperature() -> Result<f64, GenericError> {
		pros_unsafe_err_f!(battery_get_temperature, err = GenericError)
	}

	/// Get the voltage that the battery is currently supplying.
	pub fn get_voltage() -> Result<i32, GenericError> {
		pros_unsafe_err!(battery_get_voltage, err = GenericError)
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

	/// Return a bitflag of the V5 brain's current competition state
	pub fn get_status() -> CompetitionMode {
		let flags = unsafe { competition_get_status() };
		CompetitionMode::from_bits_truncate(flags)
	}

	/// Returns `true` if the V5 brain is in autonomous mode.
	pub fn is_autonomous() -> bool {
		get_status().contains(CompetitionMode::AUTONOMOUS)
	}

	/// Returns `true` if the V5 brain is disabled.
	pub fn is_disabled() -> bool {
		get_status().contains(CompetitionMode::DISABLED)
	}

	/// Returns `true` if the V5 brain is connected to the competition control.
	pub fn is_connected() -> bool {
		get_status().contains(CompetitionMode::CONNECTED)
	}
}
