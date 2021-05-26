#![no_std]
#![no_main]

extern crate pros;

#[no_mangle]
extern "C" fn initialize() {
	pros::println!("initialize()");
}

#[no_mangle]
extern "C" fn disabled() {}

#[no_mangle]
extern "C" fn competition_initialize() {}

#[no_mangle]
extern "C" fn autonomous() {}

#[no_mangle]
extern "C" fn opcontrol() {}
