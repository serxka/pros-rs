use bitflags::bitflags;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::bindings::*;
use crate::util::{get_errno, PROS_ERR, PROS_ERR_F, PROS_ERR_U32};

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

/// Describes which direction the motor is moving
#[derive(Debug)]
pub enum Direction {
	/// The motor is moving in the positive direction
	Forward,
	/// The motor is moving in the negative direction
	Reverse,
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
	pub port: u8,
}

impl Motor {
	pub fn new(
		port: u8,
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

	pub fn move_simple(&mut self, voltage: i8) -> Result<(), MotorError> {
		match unsafe { motor_move(self.port, voltage as i32) } {
			PROS_ERR => Err(MotorError::errno()),
			_ => Ok(()),
		}
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
		match unsafe { motor_move_absolute(self.port, position, velocity) } {
			PROS_ERR => Err(MotorError::errno()),
			_ => Ok(()),
		}
	}

	pub fn move_relative(
		&mut self,
		offset: f64,
		velocity: i32,
	) -> Result<(), MotorError> {
		assert!(velocity > 0);
		match unsafe { motor_move_relative(self.port, offset, velocity) } {
			PROS_ERR => Err(MotorError::errno()),
			_ => Ok(()),
		}
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
		match unsafe { motor_move_velocity(self.port, velocity) } {
			PROS_ERR => Err(MotorError::errno()),
			_ => Ok(()),
		}
	}

	pub fn move_voltage(&mut self, voltage: i16) -> Result<(), MotorError> {
		assert!(voltage >= -12000 && voltage <= 12000);
		match unsafe { motor_move_voltage(self.port, voltage as i32) } {
			PROS_ERR => Err(MotorError::errno()),
			_ => Ok(()),
		}
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
		match unsafe { motor_modify_profiled_velocity(self.port, velocity) } {
			PROS_ERR => Err(MotorError::errno()),
			_ => Ok(()),
		}
	}

	pub fn get_target_position(&mut self) -> Result<f64, MotorError> {
		let r = unsafe { motor_get_target_position(self.port) };
		if r == PROS_ERR_F {
			Err(MotorError::errno())
		} else {
			Ok(r)
		}
	}

	pub fn get_target_velocity(&mut self) -> Result<i32, MotorError> {
		match unsafe { motor_get_target_velocity(self.port) } {
			PROS_ERR => Err(MotorError::errno()),
			v => Ok(v),
		}
	}

	pub fn get_actual_velocity(&mut self) -> Result<f64, MotorError> {
		let r = unsafe { motor_get_actual_velocity(self.port) };
		if r == PROS_ERR_F {
			Err(MotorError::errno())
		} else {
			Ok(r)
		}
	}

	pub fn get_current_draw(&mut self) -> Result<u32, MotorError> {
		match unsafe { motor_get_current_draw(self.port) } {
			PROS_ERR => Err(MotorError::errno()),
			i => {
				// We can't have a negative current draw
				assert!(i >= 0);
				Ok(i as u32)
			}
		}
	}

	pub fn get_direction(&mut self) -> Result<Direction, MotorError> {
		match unsafe { motor_get_direction(self.port) } {
			PROS_ERR => Err(MotorError::errno()),
			1 => Ok(Direction::Forward),
			-1 => Ok(Direction::Reverse),
			_ => unreachable!(),
		}
	}

	pub fn get_efficiency(&mut self) -> Result<f64, MotorError> {
		let r = unsafe { motor_get_efficiency(self.port) };
		if r == PROS_ERR_F {
			Err(MotorError::errno())
		} else {
			Ok(r)
		}
	}

	pub fn get_faults(&mut self) -> Result<FaultFlags, MotorError> {
		let _f = unsafe { motor_get_faults(self.port) };
		unimplemented!()
	}

	pub fn get_flags(&mut self) -> Result<MotorFlags, MotorError> {
		let _f = unsafe { motor_get_flags(self.port) };
		unimplemented!()
	}

	pub fn get_position(&mut self) -> Result<f64, MotorError> {
		let r = unsafe { motor_get_position(self.port) };
		if r == PROS_ERR_F {
			Err(MotorError::errno())
		} else {
			Ok(r)
		}
	}

	pub fn get_power(&mut self) -> Result<f64, MotorError> {
		let r = unsafe { motor_get_power(self.port) };
		if r == PROS_ERR_F {
			Err(MotorError::errno())
		} else {
			Ok(r)
		}
	}

	pub fn get_raw_position(
		&mut self,
		timestamp: &mut u32,
	) -> Result<i32, MotorError> {
		match unsafe {
			motor_get_raw_position(self.port, timestamp as *mut u32)
		} {
			PROS_ERR => Err(MotorError::errno()),
			p => Ok(p),
		}
	}

	pub fn get_temperature(&mut self) -> Result<f64, MotorError> {
		let r = unsafe { motor_get_temperature(self.port) };
		if r == PROS_ERR_F {
			Err(MotorError::errno())
		} else {
			Ok(r)
		}
	}

	pub fn get_torque(&mut self) -> Result<f64, MotorError> {
		let r = unsafe { motor_get_torque(self.port) };
		if r == PROS_ERR_F {
			Err(MotorError::errno())
		} else {
			Ok(r)
		}
	}

	pub fn get_voltage(&mut self) -> Result<i32, MotorError> {
		match unsafe { motor_get_voltage(self.port) } {
			PROS_ERR => Err(MotorError::errno()),
			mv => Ok(mv),
		}
	}

	pub fn at_zero_position(&mut self) -> Result<bool, MotorError> {
		match unsafe { motor_get_zero_position_flag(self.port) } {
			PROS_ERR => Err(MotorError::errno()),
			1 => Ok(true),
			0 => Ok(false),
			_ => unreachable!(),
		}
	}

	pub fn is_stopped(&mut self) -> Result<bool, MotorError> {
		match unsafe { motor_is_stopped(self.port) } {
			PROS_ERR => Err(MotorError::errno()),
			1 => Ok(true),
			0 => Ok(false),
			_ => unreachable!(),
		}
	}

	pub fn is_over_current(&mut self) -> Result<bool, MotorError> {
		match unsafe { motor_is_over_current(self.port) } {
			PROS_ERR => Err(MotorError::errno()),
			1 => Ok(true),
			0 => Ok(false),
			_ => unreachable!(),
		}
	}

	pub fn is_over_temp(&mut self) -> Result<bool, MotorError> {
		match unsafe { motor_is_over_temp(self.port) } {
			PROS_ERR => Err(MotorError::errno()),
			1 => Ok(true),
			0 => Ok(false),
			_ => unreachable!(),
		}
	}

	pub fn get_brake_mode(&mut self) -> Result<BrakeMode, MotorError> {
		match unsafe { motor_get_brake_mode(self.port) } {
			PROS_ERR_U32 => Err(MotorError::errno()),
			m => match BrakeMode::from_u32(m) {
				Some(m) => Ok(m),
				None => Err(MotorError::Unknown(PROS_ERR)),
			},
		}
	}

	pub fn get_current_limit(&mut self) -> Result<u32, MotorError> {
		match unsafe { motor_get_current_limit(self.port) } {
			PROS_ERR => Err(MotorError::errno()),
			i => {
				// We can't have a negative current limit
				assert!(i >= 0);
				Ok(i as u32)
			}
		}
	}

	pub fn get_encoder_units(&mut self) -> Result<EncoderUnits, MotorError> {
		match unsafe { motor_get_encoder_units(self.port) } {
			PROS_ERR_U32 => Err(MotorError::errno()),
			m => match EncoderUnits::from_u32(m) {
				Some(m) => Ok(m),
				None => Err(MotorError::Unknown(PROS_ERR)),
			},
		}
	}

	pub fn get_gearing(&mut self) -> Result<Gearset, MotorError> {
		match unsafe { motor_get_gearing(self.port) } {
			PROS_ERR_U32 => Err(MotorError::errno()),
			m => match Gearset::from_u32(m) {
				Some(m) => Ok(m),
				None => Err(MotorError::Unknown(PROS_ERR)),
			},
		}
	}

	pub fn get_voltage_limit(&mut self) -> Result<u32, MotorError> {
		match unsafe { motor_get_voltage_limit(self.port) } {
			PROS_ERR => Err(MotorError::errno()),
			i => {
				// We can't have a negative voltage limit
				assert!(i >= 0);
				Ok(i as u32)
			}
		}
	}

	pub fn is_reversed(&mut self) -> Result<bool, MotorError> {
		match unsafe { motor_is_reversed(self.port) } {
			PROS_ERR => Err(MotorError::errno()),
			1 => Ok(true),
			0 => Ok(false),
			_ => unreachable!(),
		}
	}

	pub fn set_brake_mode(
		&mut self,
		mode: BrakeMode,
	) -> Result<(), MotorError> {
		match unsafe { motor_set_brake_mode(self.port, mode as u32) } {
			PROS_ERR => Err(MotorError::errno()),
			_ => Ok(()),
		}
	}

	pub fn set_current_limit(&mut self, limit: u32) -> Result<(), MotorError> {
		match unsafe { motor_set_current_limit(self.port, limit as i32) } {
			PROS_ERR => Err(MotorError::errno()),
			_ => Ok(()),
		}
	}

	pub fn set_encoder_units(
		&mut self,
		units: EncoderUnits,
	) -> Result<(), MotorError> {
		match unsafe { motor_set_encoder_units(self.port, units as u32) } {
			PROS_ERR => Err(MotorError::errno()),
			_ => Ok(()),
		}
	}

	pub fn set_gearing(&mut self, gearing: Gearset) -> Result<(), MotorError> {
		match unsafe { motor_set_gearing(self.port, gearing as u32) } {
			PROS_ERR => Err(MotorError::errno()),
			_ => Ok(()),
		}
	}

	pub fn set_reversed(&mut self, reverse: bool) -> Result<(), MotorError> {
		match unsafe { motor_set_reversed(self.port, reverse) } {
			PROS_ERR => Err(MotorError::errno()),
			_ => Ok(()),
		}
	}

	pub fn set_voltage_limit(&mut self, limit: u32) -> Result<(), MotorError> {
		assert!(limit <= 12000);
		match unsafe { motor_set_voltage_limit(self.port, limit as i32) } {
			PROS_ERR => Err(MotorError::errno()),
			_ => Ok(()),
		}
	}

	pub fn set_zero_postition(
		&mut self,
		position: f64,
	) -> Result<(), MotorError> {
		match unsafe { motor_set_zero_position(self.port, position) } {
			PROS_ERR => Err(MotorError::errno()),
			_ => Ok(()),
		}
	}

	pub fn tare_position(&mut self) -> Result<(), MotorError> {
		match unsafe { motor_tare_position(self.port) } {
			PROS_ERR => Err(MotorError::errno()),
			_ => Ok(()),
		}
	}
}
