use bitflags::bitflags;

use crate::bindings::*;
use crate::devices::{DeviceError, Direction};
use crate::ports::Port;

/// A struct which holds and represent a connected V5 motor
#[derive(Debug)]
pub struct Motor {
	pub port: Port,
}

impl Motor {
	/// Will create a new motor object and set some default values for it's
	/// gearset and encoder units. The break mode of the motor is defaulted to
	/// [`BrakeMode::Coast`].
	///
	/// # Panics
	/// This function will panic if a motor is not connected on this port.
	///
	/// # Safety
	/// There must only ever be a single reference to this motor. It is up to
	/// the caller to make sure there does not exists another device object with
	/// the same port. If there is another device object with the same port this
	/// will result in undefined behaviour and/or panics.
	pub unsafe fn new(
		port: Port,
		reversed: bool,
		gearset: Gearset,
		units: EncoderUnits,
	) -> Result<Self, DeviceError> {
		let mut m = Motor { port };
		m.set_brake_mode(BrakeMode::Coast)?;
		m.set_reversed(reversed)?;
		m.set_gearing(gearset)?;
		m.set_encoder_units(units)?;
		Ok(m)
	}

	#[inline]
	pub fn get_port(&self) -> u8 {
		self.port.get()
	}

	pub fn move_simple(&mut self, voltage: i8) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			motor_move,
			err = DeviceError::errno_motor(),
			self.get_port(),
			voltage as i32
		)?;
		Ok(())
	}

	pub fn stop(&mut self) -> Result<(), DeviceError> {
		self.move_velocity(0)
	}

	pub fn move_absolute(&mut self, position: f64, velocity: i32) -> Result<(), DeviceError> {
		debug_assert!(velocity > 0);
		pros_unsafe_err!(
			motor_move_absolute,
			err = DeviceError::errno_motor(),
			self.get_port(),
			position,
			velocity
		)?;
		Ok(())
	}

	pub fn move_relative(&mut self, offset: f64, velocity: i32) -> Result<(), DeviceError> {
		debug_assert!(velocity > 0);
		pros_unsafe_err!(
			motor_move_relative,
			err = DeviceError::errno_motor(),
			self.get_port(),
			offset,
			velocity
		)?;
		Ok(())
	}

	pub fn move_velocity(&mut self, velocity: i32) -> Result<(), DeviceError> {
		// Debug assertion to make sure that velocity is getting set
		// correctly
		match self.get_gearing()? {
			Gearset::Blue => debug_assert!(velocity >= -600 && velocity <= 600),
			Gearset::Green => debug_assert!(velocity >= -200 && velocity <= 200),
			Gearset::Red => debug_assert!(velocity >= -100 && velocity <= 100),
		}
		pros_unsafe_err!(
			motor_move_velocity,
			err = DeviceError::errno_motor(),
			self.get_port(),
			velocity
		)?;
		Ok(())
	}

	pub fn move_voltage(&mut self, voltage: i16) -> Result<(), DeviceError> {
		debug_assert!(voltage >= -12000 && voltage <= 12000);
		pros_unsafe_err!(
			motor_move_voltage,
			err = DeviceError::errno_motor(),
			self.get_port(),
			voltage as i32
		)?;
		Ok(())
	}

	pub fn modify_velocity(&mut self, velocity: i32) -> Result<(), DeviceError> {
		// Debug assertion to make sure that velocity is getting set
		// correctly
		match self.get_gearing()? {
			Gearset::Blue => debug_assert!(velocity >= 600 && velocity <= 600),
			Gearset::Green => debug_assert!(velocity >= 200 && velocity <= 200),
			Gearset::Red => debug_assert!(velocity >= 100 && velocity <= 100),
		}
		debug_assert!(velocity != 0);
		pros_unsafe_err!(
			motor_move_voltage,
			err = DeviceError::errno_motor(),
			self.get_port(),
			velocity
		)?;
		Ok(())
	}

	pub fn get_target_position(&self) -> Result<f64, DeviceError> {
		pros_unsafe_err_f!(
			motor_get_target_position,
			err = DeviceError::errno_motor(),
			self.get_port()
		)
	}

	pub fn get_target_velocity(&self) -> Result<i32, DeviceError> {
		pros_unsafe_err!(
			motor_get_target_velocity,
			err = DeviceError::errno_motor(),
			self.get_port()
		)
	}

	pub fn get_actual_velocity(&self) -> Result<f64, DeviceError> {
		pros_unsafe_err_f!(
			motor_get_actual_velocity,
			err = DeviceError::errno_motor(),
			self.get_port()
		)
	}

	pub fn get_current_draw(&self) -> Result<u32, DeviceError> {
		let i = pros_unsafe_err!(
			motor_get_target_velocity,
			err = DeviceError::errno_motor(),
			self.get_port()
		)?;
		Ok(i.abs() as u32)
	}

	pub fn get_direction(&self) -> Result<Direction, DeviceError> {
		let dir = pros_unsafe_err!(
			motor_get_direction,
			err = DeviceError::errno_motor(),
			self.get_port()
		)?;
		match dir {
			1 => Ok(Direction::Forward),
			-1 => Ok(Direction::Reverse),
			_ => unreachable!(),
		}
	}

	pub fn get_efficiency(&self) -> Result<f64, DeviceError> {
		pros_unsafe_err_f!(
			motor_get_efficiency,
			err = DeviceError::errno_motor(),
			self.get_port()
		)
	}

	pub fn get_faults(&self) -> Result<FaultFlags, DeviceError> {
		let _f = pros_unsafe_err_u32!(
			motor_get_faults,
			err = DeviceError::errno_motor(),
			self.get_port()
		)?;
		unimplemented!()
	}

	pub fn get_flags(&self) -> Result<MotorFlags, DeviceError> {
		let _f = pros_unsafe_err_u32!(
			motor_get_flags,
			err = DeviceError::errno_motor(),
			self.get_port()
		)?;
		unimplemented!()
	}

	pub fn get_position(&self) -> Result<f64, DeviceError> {
		pros_unsafe_err_f!(
			motor_get_position,
			err = DeviceError::errno_motor(),
			self.get_port()
		)
	}

	pub fn get_power(&self) -> Result<f64, DeviceError> {
		pros_unsafe_err_f!(
			motor_get_power,
			err = DeviceError::errno_motor(),
			self.get_port()
		)
	}

	pub fn get_raw_position(&self, mut timestamp: u32) -> Result<i32, DeviceError> {
		pros_unsafe_err!(
			motor_get_raw_position,
			err = DeviceError::errno_motor(),
			self.get_port(),
			&mut timestamp as *mut u32
		)
	}

	pub fn get_temperature(&self) -> Result<f64, DeviceError> {
		pros_unsafe_err_f!(
			motor_get_temperature,
			err = DeviceError::errno_motor(),
			self.get_port()
		)
	}

	pub fn get_torque(&self) -> Result<f64, DeviceError> {
		pros_unsafe_err_f!(
			motor_get_torque,
			err = DeviceError::errno_motor(),
			self.get_port()
		)
	}

	pub fn get_voltage(&self) -> Result<i32, DeviceError> {
		pros_unsafe_err!(
			motor_get_voltage,
			err = DeviceError::errno_motor(),
			self.get_port()
		)
	}

	pub fn at_zero_position(&self) -> Result<bool, DeviceError> {
		pros_unsafe_err_bool!(
			motor_get_zero_position_flag,
			err = DeviceError::errno_motor(),
			self.get_port()
		)
	}

	pub fn is_stopped(&self) -> Result<bool, DeviceError> {
		pros_unsafe_err_bool!(
			motor_is_stopped,
			err = DeviceError::errno_motor(),
			self.get_port()
		)
	}

	pub fn is_over_current(&self) -> Result<bool, DeviceError> {
		pros_unsafe_err_bool!(
			motor_is_over_current,
			err = DeviceError::errno_motor(),
			self.get_port()
		)
	}

	pub fn is_over_temp(&self) -> Result<bool, DeviceError> {
		pros_unsafe_err_bool!(
			motor_is_over_temp,
			err = DeviceError::errno_motor(),
			self.get_port()
		)
	}

	pub fn get_brake_mode(&self) -> Result<BrakeMode, DeviceError> {
		let m = pros_unsafe_err_u32!(
			motor_get_brake_mode,
			err = DeviceError::errno_motor(),
			self.get_port()
		)?;
		#[allow(non_upper_case_globals)]
		match m {
			motor_brake_mode_e_E_MOTOR_BRAKE_COAST => Ok(BrakeMode::Coast),
			motor_brake_mode_e_E_MOTOR_BRAKE_BRAKE => Ok(BrakeMode::Brake),
			motor_brake_mode_e_E_MOTOR_BRAKE_HOLD => Ok(BrakeMode::Hold),
			motor_brake_mode_e_E_MOTOR_BRAKE_INVALID => Err(DeviceError::errno_motor()),
			_ => panic!(
				"bindings::motor_get_brake_mode returned a value which is unknown to us: {}",
				m
			),
		}
	}

	pub fn get_encoder_units(&self) -> Result<EncoderUnits, DeviceError> {
		let m = pros_unsafe_err_u32!(
			motor_get_encoder_units,
			err = DeviceError::errno_motor(),
			self.get_port()
		)?;
		#[allow(non_upper_case_globals)]
		match m {
			motor_encoder_units_e_E_MOTOR_ENCODER_DEGREES => Ok(EncoderUnits::Degrees),
			motor_encoder_units_e_E_MOTOR_ENCODER_ROTATIONS => Ok(EncoderUnits::Rotations),
			motor_encoder_units_e_E_MOTOR_ENCODER_COUNTS => Ok(EncoderUnits::Ticks),
			motor_encoder_units_e_E_MOTOR_ENCODER_INVALID => Err(DeviceError::errno_motor()),
			_ => panic!(
				"bindings::motor_get_encoder_units returned a value which is unknown to us: {}",
				m
			),
		}
	}

	pub fn get_gearing(&self) -> Result<Gearset, DeviceError> {
		let m = pros_unsafe_err_u32!(
			motor_get_gearing,
			err = DeviceError::errno_motor(),
			self.get_port()
		)?;
		#[allow(non_upper_case_globals)]
		match m {
			motor_gearset_e_E_MOTOR_GEARSET_06 => Ok(Gearset::Blue),
			motor_gearset_e_E_MOTOR_GEARSET_18 => Ok(Gearset::Green),
			motor_gearset_e_E_MOTOR_GEARSET_36 => Ok(Gearset::Red),
			motor_gearset_e_E_MOTOR_GEARSET_INVALID => Err(DeviceError::errno_motor()),
			_ => panic!(
				"bindings::motor_get_gearing returned a value which is unknown to us: {}",
				m
			),
		}
	}

	pub fn get_current_limit(&self) -> Result<u32, DeviceError> {
		let i = pros_unsafe_err!(
			motor_get_current_limit,
			err = DeviceError::errno_motor(),
			self.get_port()
		)?;
		// We can't have a negative current limit
		debug_assert!(i >= 0);
		Ok(i as u32)
	}

	pub fn get_voltage_limit(&self) -> Result<u32, DeviceError> {
		let v = pros_unsafe_err!(
			motor_get_voltage_limit,
			err = DeviceError::errno_motor(),
			self.get_port()
		)?;
		// We can't have a negative voltage limit
		debug_assert!(v >= 0);
		Ok(v as u32)
	}

	pub fn is_reversed(&self) -> Result<bool, DeviceError> {
		pros_unsafe_err_bool!(
			motor_is_reversed,
			err = DeviceError::errno_motor(),
			self.get_port()
		)
	}

	pub fn set_brake_mode(&mut self, mode: BrakeMode) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			motor_set_brake_mode,
			err = DeviceError::errno_motor(),
			self.get_port(),
			mode as u32
		)?;
		Ok(())
	}

	pub fn set_current_limit(&mut self, limit: u32) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			motor_set_current_limit,
			err = DeviceError::errno_motor(),
			self.get_port(),
			limit as i32
		)?;
		Ok(())
	}

	pub fn set_encoder_units(&mut self, units: EncoderUnits) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			motor_set_encoder_units,
			err = DeviceError::errno_motor(),
			self.get_port(),
			units as u32
		)?;
		Ok(())
	}

	pub fn set_gearing(&mut self, gearing: Gearset) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			motor_set_gearing,
			err = DeviceError::errno_motor(),
			self.get_port(),
			gearing as u32
		)?;
		Ok(())
	}

	pub fn set_reversed(&mut self, reverse: bool) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			motor_set_reversed,
			err = DeviceError::errno_motor(),
			self.get_port(),
			reverse
		)?;
		Ok(())
	}

	pub fn set_voltage_limit(&mut self, limit: u32) -> Result<(), DeviceError> {
		debug_assert!(limit <= 12000);
		pros_unsafe_err!(
			motor_set_voltage_limit,
			err = DeviceError::errno_motor(),
			self.get_port(),
			limit as i32
		)?;
		Ok(())
	}

	pub fn set_zero_postition(&mut self, position: f64) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			motor_set_zero_position,
			err = DeviceError::errno_motor(),
			self.get_port(),
			position
		)?;
		Ok(())
	}

	pub fn tare_position(&mut self) -> Result<(), DeviceError> {
		pros_unsafe_err!(
			motor_tare_position,
			err = DeviceError::errno_motor(),
			self.get_port()
		)?;
		Ok(())
	}
}

/// Describes the behavior of the motor when braking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrakeMode {
	/// Motor coasts when stopped, traditional behavior
	Coast,
	/// Motor brakes when stopped
	Brake,
	/// Motor actively holds position when stopped
	Hold,
}

impl Default for BrakeMode {
	fn default() -> Self {
		Self::Coast
	}
}

impl From<BrakeMode> for motor_brake_mode_e {
	fn from(x: BrakeMode) -> Self {
		match x {
			BrakeMode::Coast => motor_brake_mode_e_E_MOTOR_BRAKE_COAST,
			BrakeMode::Brake => motor_brake_mode_e_E_MOTOR_BRAKE_BRAKE,
			BrakeMode::Hold => motor_brake_mode_e_E_MOTOR_BRAKE_HOLD,
		}
	}
}

/// Describes the units used when operating on the motor's encoder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncoderUnits {
	/// Position is recorded as an angle in degrees
	Degrees,
	/// Position is recorded as an angle in full rotations of the motor
	Rotations,
	/// Position is recorded as number of raw ticks from the encoder
	Ticks,
}

impl Default for EncoderUnits {
	fn default() -> Self {
		Self::Rotations
	}
}

impl From<EncoderUnits> for motor_encoder_units_e {
	fn from(x: EncoderUnits) -> Self {
		match x {
			EncoderUnits::Degrees => motor_encoder_units_e_E_MOTOR_ENCODER_DEGREES,
			EncoderUnits::Rotations => motor_encoder_units_e_E_MOTOR_ENCODER_ROTATIONS,
			EncoderUnits::Ticks => motor_encoder_units_e_E_MOTOR_ENCODER_COUNTS,
		}
	}
}

/// Describes the current gearing used on the motor. This only affects
/// calculations internally done and how input values should be interpreted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gearset {
	/// 36:1 gearing, 100RPM, Red gear set
	Red = 0,
	/// 18:1 gearing, 200RPM, Green gear set
	Green = 1,
	/// 6:1 gearing, 600RPM, Blue gear set
	Blue = 2,
}

impl From<Gearset> for motor_gearset_e {
	fn from(x: Gearset) -> Self {
		match x {
			Gearset::Blue => motor_gearset_e_E_MOTOR_GEARSET_06,
			Gearset::Green => motor_gearset_e_E_MOTOR_GEARSET_18,
			Gearset::Red => motor_gearset_e_E_MOTOR_GEARSET_36,
		}
	}
}

impl Default for Gearset {
	fn default() -> Self {
		Self::Green
	}
}

bitflags! {
	/// Describes all possible faults that could be currently occurring
	/// with the motor
	pub struct FaultFlags: u32 {
		const NONE = 0x0;
		/// Analogous to [`Motor::is_over_temp()`]
		const OVER_TEMP = 0x1;
		/// Indicates a fault with the motor H-bridge
		const DRIVER_FAULT = 0x2;
		/// Analogous to [`Motor::is_over_current()`]
		const OVER_CURRENT = 0x4;
		/// Indicates the motors H-bridge is over current
		const DRIVER_OVER_CURRENT = 0x8;
	}
}

bitflags! {
	/// Describes states that the motor can be in during operation
	pub struct MotorFlags: u32 {
		const NONE = 0x0;
		/// Cannot currently communicate to the motor
		const BUSY = 0x1;
		/// Analogous to [`Motor::is_stopped()`]
		const ZERO_VELOCITY = 0x2;
		/// Analogous to [`Motor::at_zero_position()`]
		const ZERO_POSITION = 0x4;
	}
}
