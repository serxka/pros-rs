use crate::bindings;
use crate::util::*;

use alloc::{boxed::Box, string::String};
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
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
			let slice = core::slice::from_raw_parts(name, libc::strlen(name))
				as &[u8];
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

		// take our F closure as a boxed argument for our static method to run
		// as task function
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

/// A mutual exclusion primitive useful for protecting shared date.
pub struct Mutex<T: ?Sized> {
	mutex: *mut libc::c_void,
	data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
	/// Creates a new mutex in an unlocked state ready to use.
	///
	/// # Panics
	/// This function will panic if it is unable to create the mutex.
	///
	/// # Examples
	/// ```
	/// use pros::rtos::Mutex;
	/// let mutex = Mutex::new(0);
	/// ```
	pub fn new(t: T) -> Mutex<T> {
		let m = unsafe { bindings::mutex_create() };
		if m == core::ptr::null_mut() {
			panic!("failed to create mutex");
		}

		Mutex {
			mutex: m,
			data: UnsafeCell::new(t),
		}
	}
}

impl<T: ?Sized> Mutex<T> {
	/// Acquires a mutex block the current task until it able to do so.
	///
	/// The semantics of this function are the exact same as
	/// [`Mutex::lock_timeout`] however the timeout if infinite.
	pub fn lock(&self) -> Result<MutexGuard<'_, T>, ()> {
		// A timeout of u32::MAX is the same value as `TIMEOUT_MAX` in PROS and
		// will block indefinitely
		self.lock_timeout(u32::MAX)
	}

	/// Acquires a mutex blocking the current task until it is able to do so or
	/// until the timeout is reached.
	///
	/// This function will block the current task until it is able to acquire
	/// the mutex or until the amount of time specified in milliseconds by
	/// `timeout` is reached. Upon returning this will be the only task with the
	/// lock held. An RAII guard is returned to allow scoped unlock of the lock.
	/// When the guard goes out of scope the lock will be dropped.
	///
	/// If you call `lock` on this mutex from the same task the behaviour is
	/// undefined. This function may return, there may be a panic of a deadlock.
	/// Be cautious.
	///
	/// # Errors
	/// This function will return an error if the Mutex was unable to be
	/// obtained, either due to an error or a timeout.
	pub fn lock_timeout(&self, timeout: u32) -> Result<MutexGuard<'_, T>, ()> {
		unsafe {
			if bindings::mutex_take(self.mutex, timeout) {
				Ok(MutexGuard { lock: &self })
			} else {
				Err(())
			}
		}
	}

	// /// Consumes this mutex, returning the underlying data.
	// pub fn into_inner(self) -> T
	// where
	// 	T: Sized,
	// {
	// 	let Mutex { mutex, data } = self;
	// 	unsafe { bindings::mutex_delete(mutex) };
	// 	data.into_inner()
	// }

	/// Returns a mutable reference to the underlying data.
	///
	/// Since this function calls `Mutex` mutably, no locking actually needs to
	/// take place, we are the only one with it.
	pub fn get_mut(&mut self) -> &mut T {
		self.data.get_mut()
	}
}

impl<T: ?Sized> Drop for Mutex<T> {
	fn drop(&mut self) {
		unsafe { bindings::mutex_delete(self.mutex) };
	}
}

impl<T: ?Sized + Default> Default for Mutex<T> {
	/// Creates a new unlocked `Mutex<T>` with the `Default` value for T.
	fn default() -> Mutex<T> {
		Mutex::new(Default::default())
	}
}

impl<T> From<T> for Mutex<T> {
	/// Creates a new mutex in an unlocked state ready for use. This is
	/// equivalent to [`Mutex::new`].
	fn from(t: T) -> Self {
		Mutex::new(t)
	}
}

/// A RAII implementation of a "scoped lock" mutex. When this structure is
/// dropped (it exits scope), the lock will be automatically unlocked.
///
/// The data protected by the mutex can be accessed through this guard by using
/// its `Deref` and `MutDeref` implementations.
pub struct MutexGuard<'a, T: ?Sized + 'a> {
	lock: &'a Mutex<T>,
}

impl<T: ?Sized> !Send for MutexGuard<'_, T> {}
unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
	type Target = T;

	fn deref(&self) -> &T {
		unsafe { &*self.lock.data.get() }
	}
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
	fn deref_mut(&mut self) -> &mut T {
		unsafe { &mut *self.lock.data.get() }
	}
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
	fn drop(&mut self) {
		unsafe {
			assert!(bindings::mutex_give(self.lock.mutex));
		}
	}
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
