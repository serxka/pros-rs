use crate::bindings;
use crate::util::*;

use alloc::{boxed::Box, string::String};
use core::time::Duration;

pub struct Task {
	repr: *mut core::ffi::c_void,
	name: Option<*const libc::c_char>,
}

impl Task {
	pub const STACK_DEFAULT_SIZE: u16 = 0x2000;
	pub const STACK_MINIMUM_SIZE: u16 = 0x200;

	pub const PRIORITY_DEFAULT: u32 = 8;
	pub const PRIORITY_MIN: u32 = 1;
	pub const PRIORITY_MAX: u32 = 16;

	pub fn name<'a>(&mut self) -> &'a str {
		if self.name.is_none() {
			self.name = Some(unsafe { bindings::task_get_name(self.repr) });
		}

		let name = self.name.unwrap();
		unsafe {
			let slice = core::slice::from_raw_parts(name, libc::strlen(name)) as &[u8];
			core::str::from_utf8(slice).unwrap()
		}
	}

	pub fn resume(&self) {
		unsafe {
			bindings::task_resume(self.repr);
		}
	}

	pub fn suspend(&self) {
		unsafe {
			bindings::task_suspend(self.repr);
		}
	}
}

pub fn current() -> Task {
	unsafe {
		let name = bindings::task_get_name(core::ptr::null_mut());
		let id = bindings::task_get_by_name(name);

		if id == core::ptr::null_mut() {
			panic!("failed to get current thread!");
		}

		Task {
			repr: id,
			name: Some(name),
		}
	}
}

pub struct TaskBuilder {
	name: Option<String>,
	stack_size: Option<u16>,
	priority: Option<u32>,
}

impl TaskBuilder {
	pub fn new() -> TaskBuilder {
		TaskBuilder {
			name: None,
			stack_size: None,
			priority: None,
		}
	}

	pub fn name(mut self, name: String) -> TaskBuilder {
		self.name = Some(name);
		self
	}

	pub fn stack_size(mut self, stack_size: u16) -> TaskBuilder {
		self.stack_size = Some(stack_size);
		self
	}

	pub fn priority(mut self, priority: u32) -> TaskBuilder {
		self.priority = Some(priority);
		self
	}

	pub fn spawn<F: FnOnce() + Send + 'static>(self, f: F) -> Result<Task, ()> {
		let stack_size = u16::max(
			self.stack_size.unwrap_or(Task::STACK_DEFAULT_SIZE),
			Task::STACK_MINIMUM_SIZE,
		);
		let priority = self
			.priority
			.unwrap_or(Task::PRIORITY_DEFAULT)
			.clamp(Task::PRIORITY_MIN, Task::PRIORITY_MAX);
		let name = to_cstring(self.name.unwrap_or_else(|| String::from(" ")));

		// take our F closure as a boxed argument for our static method to run as task function
		extern "C" fn run<F: FnOnce()>(arg: *mut libc::c_void) {
			let boxed_cb = unsafe { Box::from_raw(arg as *mut F) };
			boxed_cb();
		}

		let cb = Box::new(f);
		unsafe {
			let arg = Box::into_raw(cb);
			let res = bindings::task_create(
				Some(run::<F>),
				arg as *mut libc::c_void,
				priority,
				stack_size,
				name.as_ptr(),
			);
			if res == core::ptr::null_mut() {
				Box::from_raw(arg); // rebox pointer to avoid leak if failed to create task
					// TODO: error handling
				Err(())
			} else {
				Ok(Task {
					repr: res,
					name: None,
				})
			}
		}
	}
}

pub fn spawn<F: FnOnce() + Send + 'static>(f: F) -> Task {
	TaskBuilder::new().spawn(f).expect("failed to spawn task")
}

pub struct Instant(Duration);

impl Instant {
	pub fn now() -> Instant {
		let micros = unsafe { bindings::micros() };
		Instant(Duration::from_micros(micros))
	}

	pub fn elapsed(&self) -> Duration {
		Instant::now().0 - (*self).0
	}

	pub fn duration_since(&self, ealier: Instant) -> Duration {
		ealier.0 - self.0
	}
}

pub fn delay(dur: Duration) {
	unsafe { bindings::delay(dur.as_millis() as u32) }
}
