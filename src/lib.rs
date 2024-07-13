#![no_std]
#![feature(negative_impls)]
#![feature(const_option)]

//! # PROS bindings
//! This library contains safe Rust bindings for the Vex PROS environment. It should be used in conjunction with this template crate [github.com/serxka/pros-rs-template](https://github.com/serxka/pros-rs-template).
//!
//! This library is currently a work in progress, expect breaking API changes or
//! undefined behaviour, or just straight up missing features.
//!
//! # Hello World
//! ```
//! #![no_std]
//! #![no_main]
//!
//! extern crate pros;
//! use pros::prelude::*;
//!
//! struct VexRobot;
//!
//! impl Robot for VexRobot {
//! 	fn new(devices: Devices) -> Self {
//! 		println!("Hello World!");
//! 		VexRobot
//! 	}
//! }
//! robot!(VexRobot);
//! ```

extern crate alloc;
#[macro_use]
extern crate smallvec;

mod bindings {
	pub use pros_sys::*;
}

#[doc(hidden)]
#[macro_use]
pub mod macros;
mod util;

pub mod devices;
pub mod ports;
pub mod prelude {
	//! Common types and macros that can all be conveniently imported at once.

	pub use crate::devices::{controller::*, motor::*, Colour, DeviceError, Devices};
	pub use crate::ports::*;
	pub use crate::rtos::{
		action::{Action, NextSleep, Poll},
		tasks::{CompetitionState, CompetitionTask, Task},
		time::{Instant, Interval},
		Mutex,
	};
	pub use crate::Robot;
	pub use crate::{action, robot};
	pub use alloc::vec::Vec;
	pub use core::time::Duration;
	pub use libc_print::std_name::*;
}
pub mod rtos;

/// This trait is used so that `pros-rs` knows which functions it should call
/// for the tasks that are addressed out by the competition manager.
///
/// This means that every single one of these functions will be called from a
/// separate task and that they can return early at any possible time. The
/// exception to this is [`Robot::new()`] which will block all other functions
/// from being called until it returns.
pub trait Robot {
	/// The entry point to the users program, a structure containing owned
	/// values to every possible device is passed into this function. The ports
	/// that will be needed throughout the whole of the programs execution
	/// should be moved and called with the relevant `into_<>` function to
	/// convert them into specific devices.
	///
	/// [`Robot::new()`] should ideally return as soon as possible to allow
	/// operator control and autonomous code to run as soon as possible. This
	/// means keeping the initialisation code lean and not waiting around
	/// collecting sensor data. Collection of sensor data should be performed
	/// from another task.
	fn new(devices: devices::Devices) -> Self;

	#[allow(unused_variables)]
	fn competition_init(&'static self, state: rtos::tasks::CompetitionState) {}

	#[allow(unused_variables)]
	fn disabled(&'static self, state: rtos::tasks::CompetitionState) {}

	#[allow(unused_variables)]
	fn autonomous(&'static self, state: rtos::tasks::CompetitionState) {}

	#[allow(unused_variables)]
	fn opcontrol(&'static self, state: rtos::tasks::CompetitionState) {}
}

// TODO: Printing to screen for easy debugging
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	// Print main text
	libc_print::libc_eprint!("task panicked at ");
	// Print panic message
	if let Some(s) = info.payload().downcast_ref::<&str>() {
		libc_print::libc_eprint!("'{}', ", s);
	} else {
		let args = info.message();
		libc_print::libc_eprint!("'{}', ", args);
	}
	// Print panic location
	if let Some(s) = info.location() {
		libc_print::libc_eprint!("{}:{}", s.file(), s.line());
	} else {
		libc_print::libc_eprint!("<unknown location>");
	}
	libc_print::libc_eprintln!();

	// TODO: format panic message for screen
	//	screen_print_at(0, cstr!("panicked!"));

	if let Some(s) = info.location() {
		screen_print_at(
			0,
			alloc::ffi::CString::new(alloc::format!(
				"panicked at {}:{}",
				s.line(),
				&s.file()[s.file().len() - 10..]
			))
			.unwrap()
			.as_c_str()
			.as_ptr() as _,
		);
	} else {
		screen_print_at(0, cstr!("panicked"));
	}

	unsafe {
		// Go through and stop motors regardless if they are actually motors or not
		for i in 1..21 {
			bindings::motor_set_brake_mode(i, devices::motor::BrakeMode::Coast.into());
			bindings::motor_move_velocity(i, 0);
		}
	}
	loop {}
}

extern "C" {
	// enum abi mismatch
	fn screen_print(txt_fmt: u8, line: i16, text: *const core::ffi::c_char, ...) -> u32;
}

pub fn screen_print_at(line: u8, msg: *const u8) {
	unsafe {
		screen_print(
			bindings::text_format_e_t_E_TEXT_MEDIUM as _,
			line as _,
			cstr!("%s") as *const _,
			msg,
		);
	}
}
