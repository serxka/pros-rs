use bitflags::bitflags;
use smallvec::SmallVec;

use crate::bindings::*;
use crate::devices::DeviceError;
use crate::util::to_cstring;

use alloc::string::String;

/// A reference to a certain connected controller.
#[derive(Debug)]
pub struct Controller {
	id: u32,
}

impl Controller {
	/// Get a reference to the master controller on the V5 Brain.
	///
	/// # Safety
	/// This function is unsafe as there must only ever be a single reference to
	/// a controller. Creating a new reference to a controller and calling
	/// functions on it may result in undefined behaviour.
	pub unsafe fn master() -> Controller {
		Controller {
			id: controller_id_e_t_E_CONTROLLER_MASTER,
		}
	}

	/// Get a reference to the slave controller on the V5 Brain.
	///
	/// # Safety
	/// This function is unsafe as there must only ever be a single reference to
	/// a controller. Creating a new reference to a controller and calling
	/// functions on it may result in undefined behaviour.
	pub unsafe fn slave() -> Controller {
		Controller {
			id: controller_id_e_t_E_CONTROLLER_PARTNER,
		}
	}

	/// Gets the value of an analog axis (joystick) on a controller. As an `i8`
	/// value, if you would like the value as a float use
	/// [`Controller::get_analog`].
	pub fn get_analog_raw(&self, axis: Axis) -> Result<i8, DeviceError> {
		let res = pros_unsafe_err!(
			controller_get_analog,
			err = DeviceError::errno_generic(),
			self.id,
			axis.into()
		)?;
		Ok(res as i8)
	}

	/// Gets the value of an analog axis (joystick) on a controller.
	pub fn get_analog(&self, axis: Axis) -> Result<f64, DeviceError> {
		self.get_analog_raw(axis)
			.map(|i| (i as f64 / 127.0).clamp(-1.0, 1.0))
	}

	/// Get the value of a digital axis (button) on a controller. If the axis is
	/// high a `true` boolean is return, likewise if it low a `false` is
	/// returned.
	pub fn get_button(&self, button: Button) -> Result<bool, DeviceError> {
		pros_unsafe_err_bool!(
			controller_get_digital,
			err = DeviceError::errno_generic(),
			self.id,
			button.into()
		)
	}

	/// Get the value of a digital axis (button) on a controller when it goes
	/// high (positive-edge). If the axis just when high a `true` boolean is
	/// return, likewise if it is low or didn't just go high a `false` is
	/// returned.
	pub fn get_button_new_press(&self, button: Button) -> Result<bool, DeviceError> {
		pros_unsafe_err_bool!(
			controller_get_digital_new_press,
			err = DeviceError::errno_generic(),
			self.id,
			button.into()
		)
	}

	/// Gets the battery capacity for the given controller.
	pub fn battery_capacity(&self) -> Result<i32, DeviceError> {
		pros_unsafe_err!(
			controller_get_battery_capacity,
			err = DeviceError::errno_generic(),
			self.id
		)
	}

	/// Gets the battery level for the given controller.
	pub fn battery_level(&self) -> Result<i32, DeviceError> {
		pros_unsafe_err!(
			controller_get_battery_level,
			err = DeviceError::errno_generic(),
			self.id
		)
	}

	/// Tests to see if this controller is currently connected or not.
	pub fn is_connected(&self) -> Result<bool, DeviceError> {
		pros_unsafe_err_bool!(
			controller_is_connected,
			err = DeviceError::errno_generic(),
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

/// Which axis to read when calling [`Controller::get_analog`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
	/// The left to right axis of the left joystick,
	LeftX,
	/// The down to up axis of the left joystick,
	LeftY,
	/// The left to right axis of the right joystick,
	RightX,
	/// The down to up axis of the right joystick,
	RightY,
}

impl From<Axis> for controller_analog_e_t {
	fn from(x: Axis) -> Self {
		match x {
			Axis::LeftX => controller_analog_e_t_E_CONTROLLER_ANALOG_LEFT_X,
			Axis::LeftY => controller_analog_e_t_E_CONTROLLER_ANALOG_LEFT_Y,
			Axis::RightX => controller_analog_e_t_E_CONTROLLER_ANALOG_RIGHT_X,
			Axis::RightY => controller_analog_e_t_E_CONTROLLER_ANALOG_RIGHT_Y,
		}
	}
}

/// Which button to read when calling [`Controller::get_button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
	L1,
	L2,
	R1,
	R2,
	Up,
	Down,
	Left,
	Right,
	X,
	B,
	Y,
	A,
}

impl From<Button> for controller_digital_e_t {
	fn from(x: Button) -> Self {
		match x {
			Button::L1 => controller_digital_e_t_E_CONTROLLER_DIGITAL_L1,
			Button::L2 => controller_digital_e_t_E_CONTROLLER_DIGITAL_L2,
			Button::R1 => controller_digital_e_t_E_CONTROLLER_DIGITAL_R1,
			Button::R2 => controller_digital_e_t_E_CONTROLLER_DIGITAL_R2,
			Button::Up => controller_digital_e_t_E_CONTROLLER_DIGITAL_UP,
			Button::Down => controller_digital_e_t_E_CONTROLLER_DIGITAL_DOWN,
			Button::Left => controller_digital_e_t_E_CONTROLLER_DIGITAL_LEFT,
			Button::Right => controller_digital_e_t_E_CONTROLLER_DIGITAL_RIGHT,
			Button::X => controller_digital_e_t_E_CONTROLLER_DIGITAL_X,
			Button::B => controller_digital_e_t_E_CONTROLLER_DIGITAL_B,
			Button::Y => controller_digital_e_t_E_CONTROLLER_DIGITAL_Y,
			Button::A => controller_digital_e_t_E_CONTROLLER_DIGITAL_A,
		}
	}
}

#[derive(Debug)]
/// An empty struct containing methods to get the status of the battery.
pub struct Battery;

impl Battery {
	/// Get the current capacity of the battery.
	pub fn get_capacity() -> Result<f64, DeviceError> {
		pros_unsafe_err_f!(battery_get_capacity, err = DeviceError::errno_generic())
	}

	/// Get the amount of current that is currently being drawn from the
	/// battery.
	pub fn get_current() -> Result<i32, DeviceError> {
		pros_unsafe_err!(battery_get_current, err = DeviceError::errno_generic())
	}

	/// Get the temperature of the battery. This is helpful for supplying any
	/// warnings.
	pub fn get_temperature() -> Result<f64, DeviceError> {
		pros_unsafe_err_f!(battery_get_temperature, err = DeviceError::errno_generic())
	}

	/// Get the voltage that the battery is currently supplying.
	pub fn get_voltage() -> Result<i32, DeviceError> {
		pros_unsafe_err!(battery_get_voltage, err = DeviceError::errno_generic())
	}
}

bitflags! {
	/// Bitflags for defining the state of the robot in competition mode.
	pub struct CompetitionMode: u8 {
		/// The brain is disabled.
		const DISABLED = 0x1 << 0x0;
		/// The brain is in autonomous mode.
		const AUTONOMOUS = 0x1 << 0x1;
		/// The brain is connected to the competition control.
		const CONNECTED = 0x1 << 0x2;
	}
}

impl CompetitionMode {
	/// Returns a bitflag of the V5 Brain's current competition state.
	pub fn get_status() -> CompetitionMode {
		let flags = unsafe { competition_get_status() };
		CompetitionMode::from_bits_truncate(flags)
	}

	/// Returns `true` if the V5 Brain is in autonomous mode.
	pub fn is_autonomous() -> bool {
		Self::get_status().contains(CompetitionMode::AUTONOMOUS)
	}

	/// Returns `true` if the V5 Brain is disabled.
	pub fn is_disabled() -> bool {
		Self::get_status().contains(CompetitionMode::DISABLED)
	}

	/// Returns `true` if the V5 Brain is connected to the competition control.
	pub fn is_connected() -> bool {
		Self::get_status().contains(CompetitionMode::CONNECTED)
	}
}
