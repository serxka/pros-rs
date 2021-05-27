#![no_std]
#![no_main]

#[macro_use]
extern crate pros;
use pros::prelude::*;

struct VexRobot;

impl Robot for VexRobot {
	fn new(/*registry: Registry*/) -> Self {
		VexRobot
	}

	fn competition_init(&mut self) {
		println!("competition_init()");
	}

	fn disabled(&mut self) {
		println!("disabled()");
	}

	fn autonomous(&mut self) {
		println!("autonomous()");
	}

	fn opcontrol(&mut self) {
		println!("opcontrol()");
	}
}

robot!(VexRobot);
