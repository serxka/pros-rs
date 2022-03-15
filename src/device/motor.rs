use bitflags::bitflags;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::bindings::*;
use crate::device::Direction;
use crate::util::{get_errno, Port, PROS_ERR};

/// Possible errors that could be returned from motor function calls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorError {
	/// The Port chosen cannot be configured as a motor
	PortNotMotor,
	/// The Port chosen is not within the range of supported ports of the
	/// V5 Brain
	PortRange,
	/// An unknown error
	#[doc(hidden)]
	Unknown(i32),
}

impl MotorError {
	pub(crate) fn errno() -> Self {
		match get_errno() {
			libc::ENODEV => Self::PortNotMotor,
			libc::ENXIO => Self::PortRange,
			e => Self::Unknown(e),
		}
	}
}

/// Describes the behavior of the motor when braking.
#[repr(u32)]
#[derive(Debug, FromPrimitive, Clone, Copy, PartialEq, Eq)]
pub enum BrakeMode {
	/// Motor coasts when stopped, traditional behavior
	Coast = motor_brake_mode_e_E_MOTOR_BRAKE_COAST,
	/// Motor brakes when stopped
	Brake = motor_brake_mode_e_E_MOTOR_BRAKE_BRAKE,
	/// Motor actively holds position when stopped
	Hold = motor_brake_mode_e_E_MOTOR_BRAKE_HOLD,
}

/// Describes the units used when operating on the motor's encoder.
#[repr(u32)]
#[derive(Debug, FromPrimitive, Clone, Copy, PartialEq, Eq)]
pub enum EncoderUnits {
	/// Position is recorded as an angle in degrees
	Degrees = motor_encoder_units_e_E_MOTOR_ENCODER_DEGREES,
	/// Position is recorded as an angle in full rotations of the motor
	Rotations = motor_encoder_units_e_E_MOTOR_ENCODER_ROTATIONS,
	/// Position is recorded as number of raw ticks from the encoder
	Ticks = motor_encoder_units_e_E_MOTOR_ENCODER_COUNTS,
}

/// Describes the current gearing used on the motor. This only affects
/// calculations internally done and how input values should be interpreted.
#[repr(u32)]
#[derive(Debug, FromPrimitive, Clone, Copy, PartialEq, Eq)]
pub enum Gearset {
	/// 6:1 gearing, 600RPM, Blue gear set
	Blue = motor_gearset_e_E_MOTOR_GEARSET_06,
	/// 18:1 gearing, 200RPM, Green gear set
	Green = motor_gearset_e_E_MOTOR_GEARSET_18,
	/// 36:1 gearing, 100RPM, Red gear set
	Red = motor_gearset_e_E_MOTOR_GEARSET_36,
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
		/// Analogous to [`Motor::get_zero_position_flag()`]
		const ZERO_POSITION = 0x4;
	}
}

/// A struct which holds and represent a connected V5 motor
#[derive(Debug)]
pub struct Motor {
	pub port: Port,
}

impl Motor {
	pub fn new(
		port: Port,
		reversed: bool,
		gearset: Gearset,
		units: EncoderUnits,
	) -> Result<Self, MotorError> {
		let mut m = Motor { port };
		m.set_brake_mode(BrakeMode::Coast)?;
		m.set_reversed(reversed)?;
		m.set_gearing(gearset)?;
		m.set_encoder_units(units)?;
		Ok(m)
	}

	fn get_port(&self) -> u8 {
		self.port.get()
	}

	pub fn move_simple(&mut self, voltage: i8) -> Result<(), MotorError> {
		pros_unsafe_err!(
			motor_move,
			err = MotorError,
			self.get_port(),
			voltage as i32
		)?;
		Ok(())
	}

	pub fn stop(&mut self) -> Result<(), MotorError> {
		self.move_simple(0)
	}

	pub fn move_absolute(
		&mut self,
		position: f64,
		velocity: i32,
	) -> Result<(), MotorError> {
		assert!(velocity > 0);
		pros_unsafe_err!(
			motor_move_absolute,
			err = MotorError,
			self.get_port(),
			position,
			velocity
		)?;
		Ok(())
	}

	pub fn move_relative(
		&mut self,
		offset: f64,
		velocity: i32,
	) -> Result<(), MotorError> {
		assert!(velocity > 0);
		pros_unsafe_err!(
			motor_move_relative,
			err = MotorError,
			self.get_port(),
			offset,
			velocity
		)?;
		Ok(())
	}

	pub fn move_velocity(&mut self, velocity: i32) -> Result<(), MotorError> {
		// Debug assertion to make sure that velocity is getting set
		// correctly
		#[cfg(debug_assertions)]
		match self.get_gearing()? {
			Gearset::Blue => assert!(velocity >= 600 && velocity <= 600),
			Gearset::Green => assert!(velocity >= 200 && velocity <= 200),
			Gearset::Red => assert!(velocity >= 100 && velocity <= 100),
		}
		assert!(velocity != 0);
		pros_unsafe_err!(
			motor_move_velocity,
			err = MotorError,
			self.get_port(),
			velocity
		)?;
		Ok(())
	}

	pub fn move_voltage(&mut self, voltage: i16) -> Result<(), MotorError> {
		assert!(voltage >= -12000 && voltage <= 12000);
		pros_unsafe_err!(
			motor_move_voltage,
			err = MotorError,
			self.get_port(),
			voltage as i32
		)?;
		Ok(())
	}

	pub fn modify_velocity(&mut self, velocity: i32) -> Result<(), MotorError> {
		// Debug assertion to make sure that velocity is getting set
		// correctly
		#[cfg(debug_assertions)]
		match self.get_gearing()? {
			Gearset::Blue => assert!(velocity >= 600 && velocity <= 600),
			Gearset::Green => assert!(velocity >= 200 && velocity <= 200),
			Gearset::Red => assert!(velocity >= 100 && velocity <= 100),
		}
		assert!(velocity != 0);
		pros_unsafe_err!(
			motor_move_voltage,
			err = MotorError,
			self.get_port(),
			velocity
		)?;
		Ok(())
	}

	pub fn get_target_position(&mut self) -> Result<f64, MotorError> {
		pros_unsafe_err_f!(
			motor_get_target_position,
			err = MotorError,
			self.get_port()
		)
	}

	pub fn get_target_velocity(&mut self) -> Result<i32, MotorError> {
		pros_unsafe_err!(
			motor_get_target_velocity,
			err = MotorError,
			self.get_port()
		)
	}

	pub fn get_actual_velocity(&mut self) -> Result<f64, MotorError> {
		pros_unsafe_err_f!(
			motor_get_actual_velocity,
			err = MotorError,
			self.get_port()
		)
	}

	pub fn get_current_draw(&mut self) -> Result<u32, MotorError> {
		let i = pros_unsafe_err!(
			motor_get_target_velocity,
			err = MotorError,
			self.get_port()
		)?;
		// We can't have a negative current draw
		assert!(i >= 0);
		Ok(i as u32)
	}

	pub fn get_direction(&mut self) -> Result<Direction, MotorError> {
		let dir = pros_unsafe_err!(
			motor_get_direction,
			err = MotorError,
			self.get_port()
		)?;
		match dir {
			1 => Ok(Direction::Forward),
			-1 => Ok(Direction::Reverse),
			_ => unreachable!(),
		}
	}

	pub fn get_efficiency(&mut self) -> Result<f64, MotorError> {
		pros_unsafe_err_f!(
			motor_get_efficiency,
			err = MotorError,
			self.get_port()
		)
	}

	pub fn get_faults(&mut self) -> Result<FaultFlags, MotorError> {
		let _f = pros_unsafe_err_u32!(
			motor_get_faults,
			err = MotorError,
			self.get_port()
		)?;
		unimplemented!()
	}

	pub fn get_flags(&mut self) -> Result<MotorFlags, MotorError> {
		let _f = pros_unsafe_err_u32!(
			motor_get_flags,
			err = MotorError,
			self.get_port()
		)?;
		unimplemented!()
	}

	pub fn get_position(&mut self) -> Result<f64, MotorError> {
		pros_unsafe_err_f!(
			motor_get_position,
			err = MotorError,
			self.get_port()
		)
	}

	pub fn get_power(&mut self) -> Result<f64, MotorError> {
		pros_unsafe_err_f!(motor_get_power, err = MotorError, self.get_port())
	}

	pub fn get_raw_position(
		&mut self,
		mut timestamp: u32,
	) -> Result<i32, MotorError> {
		pros_unsafe_err!(
			motor_get_raw_position,
			err = MotorError,
			self.get_port(),
			&mut timestamp as *mut u32
		)
	}

	pub fn get_temperature(&mut self) -> Result<f64, MotorError> {
		pros_unsafe_err_f!(
			motor_get_temperature,
			err = MotorError,
			self.get_port()
		)
	}

	pub fn get_torque(&mut self) -> Result<f64, MotorError> {
		pros_unsafe_err_f!(motor_get_torque, err = MotorError, self.get_port())
	}

	pub fn get_voltage(&mut self) -> Result<i32, MotorError> {
		pros_unsafe_err!(motor_get_voltage, err = MotorError, self.get_port())
	}

	pub fn at_zero_position(&mut self) -> Result<bool, MotorError> {
		pros_unsafe_err_bool!(
			motor_get_zero_position_flag,
			err = MotorError,
			self.get_port()
		)
	}

	pub fn is_stopped(&mut self) -> Result<bool, MotorError> {
		pros_unsafe_err_bool!(
			motor_is_stopped,
			err = MotorError,
			self.get_port()
		)
	}

	pub fn is_over_current(&mut self) -> Result<bool, MotorError> {
		pros_unsafe_err_bool!(
			motor_is_over_current,
			err = MotorError,
			self.get_port()
		)
	}

	pub fn is_over_temp(&mut self) -> Result<bool, MotorError> {
		pros_unsafe_err_bool!(
			motor_is_over_temp,
			err = MotorError,
			self.get_port()
		)
	}

	pub fn get_brake_mode(&mut self) -> Result<BrakeMode, MotorError> {
		let m = pros_unsafe_err_u32!(
			motor_get_brake_mode,
			err = MotorError,
			self.get_port()
		)?;
		match BrakeMode::from_u32(m) {
			Some(m) => Ok(m),
			None => Err(MotorError::Unknown(PROS_ERR)),
		}
	}

	pub fn get_current_limit(&mut self) -> Result<u32, MotorError> {
		let i = pros_unsafe_err!(
			motor_get_current_limit,
			err = MotorError,
			self.get_port()
		)?;
		// We can't have a negative current limit
		assert!(i >= 0);
		Ok(i as u32)
	}

	pub fn get_encoder_units(&mut self) -> Result<EncoderUnits, MotorError> {
		let m = pros_unsafe_err_u32!(
			motor_get_encoder_units,
			err = MotorError,
			self.get_port()
		)?;
		match EncoderUnits::from_u32(m) {
			Some(m) => Ok(m),
			None => Err(MotorError::Unknown(PROS_ERR)),
		}
	}

	pub fn get_gearing(&mut self) -> Result<Gearset, MotorError> {
		let m = pros_unsafe_err_u32!(
			motor_get_gearing,
			err = MotorError,
			self.get_port()
		)?;
		match Gearset::from_u32(m) {
			Some(m) => Ok(m),
			None => Err(MotorError::Unknown(PROS_ERR)),
		}
	}

	pub fn get_voltage_limit(&mut self) -> Result<u32, MotorError> {
		let v = pros_unsafe_err!(
			motor_get_voltage_limit,
			err = MotorError,
			self.get_port()
		)?;
		// We can't have a negative voltage limit
		assert!(v >= 0);
		Ok(v as u32)
	}

	pub fn is_reversed(&mut self) -> Result<bool, MotorError> {
		pros_unsafe_err_bool!(
			motor_is_reversed,
			err = MotorError,
			self.get_port()
		)
	}

	pub fn set_brake_mode(
		&mut self,
		mode: BrakeMode,
	) -> Result<(), MotorError> {
		pros_unsafe_err!(
			motor_set_brake_mode,
			err = MotorError,
			self.get_port(),
			mode as u32
		)?;
		Ok(())
	}

	pub fn set_current_limit(&mut self, limit: u32) -> Result<(), MotorError> {
		pros_unsafe_err!(
			motor_set_current_limit,
			err = MotorError,
			self.get_port(),
			limit as i32
		)?;
		Ok(())
	}

	pub fn set_encoder_units(
		&mut self,
		units: EncoderUnits,
	) -> Result<(), MotorError> {
		pros_unsafe_err!(
			motor_set_encoder_units,
			err = MotorError,
			self.get_port(),
			units as u32
		)?;
		Ok(())
	}

	pub fn set_gearing(&mut self, gearing: Gearset) -> Result<(), MotorError> {
		pros_unsafe_err!(
			motor_set_gearing,
			err = MotorError,
			self.get_port(),
			gearing as u32
		)?;
		Ok(())
	}

	pub fn set_reversed(&mut self, reverse: bool) -> Result<(), MotorError> {
		pros_unsafe_err!(
			motor_set_reversed,
			err = MotorError,
			self.get_port(),
			reverse
		)?;
		Ok(())
	}

	pub fn set_voltage_limit(&mut self, limit: u32) -> Result<(), MotorError> {
		assert!(limit <= 12000);
		pros_unsafe_err!(
			motor_set_voltage_limit,
			err = MotorError,
			self.get_port(),
			limit as i32
		)?;
		Ok(())
	}

	pub fn set_zero_postition(
		&mut self,
		position: f64,
	) -> Result<(), MotorError> {
		pros_unsafe_err!(
			motor_set_zero_position,
			err = MotorError,
			self.get_port(),
			position
		)?;
		Ok(())
	}

	pub fn tare_position(&mut self) -> Result<(), MotorError> {
		pros_unsafe_err!(
			motor_tare_position,
			err = MotorError,
			self.get_port()
		)?;
		Ok(())
	}
}
