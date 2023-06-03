//! Functionally relevant to PROS RTOS multitasking, also contains time keeping
//! and synchronisation primitives.

pub mod action;
pub mod tasks;
pub mod time;

use crate::bindings;
use crate::devices::DeviceError;

use core::{
	cell::UnsafeCell,
	mem::MaybeUninit,
	ops::{Deref, DerefMut},
	sync::atomic::{AtomicBool, Ordering},
	time::Duration,
};

struct MutexInner {
	ptr: *mut libc::c_void,
}

impl MutexInner {
	pub fn new() -> MutexInner {
		Self::try_new().expect("failed to create mutex")
	}

	pub fn try_new() -> Result<MutexInner, DeviceError> {
		let ptr = unsafe { bindings::mutex_create() };
		if ptr == core::ptr::null_mut() {
			Err(DeviceError::errno_generic())
		} else {
			Ok(MutexInner { ptr })
		}
	}

	pub fn take(&self, timeout: Duration) -> bool {
		dbg_duration_is_u32!(timeout);
		unsafe { bindings::mutex_take(self.ptr, timeout.as_millis() as u32) }
	}

	pub fn give(&self) -> bool {
		unsafe { bindings::mutex_give(self.ptr) }
	}
}

impl Drop for MutexInner {
	fn drop(&mut self) {
		unsafe { bindings::mutex_delete(self.ptr) };
	}
}

/// A mutual exclusion primitive useful for protecting shared date.
pub struct Mutex<T: ?Sized> {
	mutex: MutexInner,
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
		Mutex {
			mutex: MutexInner::new(),
			data: UnsafeCell::new(t),
		}
	}
}

impl<T: ?Sized> Mutex<T> {
	/// Acquires a mutex block the current task until it able to do so.
	///
	/// The semantics of this function are the exact same as
	/// [`Mutex::lock_timeout`] however the timeout if infinite.
	pub fn lock(&self) -> MutexGuard<'_, T> {
		// A timeout of u32::MAX is the same value as `TIMEOUT_MAX` in PROS and
		// will block indefinitely, this is why `None` should be unreachable
		match self.lock_timeout(time::INF_TIMEOUT) {
			Some(guard) => guard,
			None => unreachable!(),
		}
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
	/// This function will return an option if the Mutex was unable to be
	/// obtained due to a timeout.
	pub fn lock_timeout(&self, timeout: Duration) -> Option<MutexGuard<'_, T>> {
		if self.mutex.take(timeout) {
			Some(MutexGuard { lock: &self })
		} else {
			None
		}
	}

	/// Consumes this mutex, returning the underlying data.
	pub fn into_inner(self) -> T
	where
		T: Sized,
	{
		self.data.into_inner()
	}

	/// Returns a mutable reference to the underlying data.
	///
	/// Since this function calls `Mutex` mutably, no locking actually needs to
	/// take place, we are the only one with it.
	pub fn get_mut(&mut self) -> &mut T {
		self.data.get_mut()
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
		debug_assert!(self.lock.mutex.give());
	}
}

pub struct Semaphore {
	ptr: *mut libc::c_void,
}

unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}

impl Semaphore {
	pub fn new(max_count: u32, init_count: u32) -> Semaphore {
		Self::new_try(max_count, init_count).expect("failed to create semaphore")
	}

	pub fn new_try(max_count: u32, init_count: u32) -> Result<Semaphore, DeviceError> {
		let ptr = unsafe { bindings::sem_create(max_count, init_count) };
		if ptr == core::ptr::null_mut() {
			Err(DeviceError::errno_generic())
		} else {
			Ok(Semaphore { ptr })
		}
	}

	pub fn wait(&self) -> bool {
		self.wait_timeout(time::INF_TIMEOUT)
	}

	pub fn poll(&self) -> bool {
		self.wait_timeout(Duration::ZERO)
	}

	pub fn wait_timeout(&self, timeout: Duration) -> bool {
		dbg_duration_is_u32!(timeout);
		unsafe { bindings::sem_wait(self.ptr, timeout.as_millis() as u32) }
	}

	pub fn post(&self) -> Result<(), DeviceError> {
		if unsafe { bindings::mutex_give(self.ptr) } {
			Err(DeviceError::errno_generic())
		} else {
			Ok(())
		}
	}

	pub fn count(&self) -> usize {
		unsafe { bindings::sem_get_count(self.ptr) as usize }
	}
}

impl Drop for Semaphore {
	fn drop(&mut self) {
		unsafe { bindings::sem_delete(self.ptr) };
	}
}

pub struct OnceCell<T> {
	has_init: AtomicBool,
	item: MaybeUninit<T>,
}

impl<T> OnceCell<T> {
	pub const fn new() -> Self {
		Self {
			has_init: AtomicBool::new(false),
			item: MaybeUninit::uninit(),
		}
	}

	/// This function will only ever be called once
	pub fn call_once<F: FnOnce() -> T>(&self, f: F) {
		let s = unsafe { &mut *(self as *const Self as *mut Self) };

		if s.has_init.load(Ordering::Acquire) {
			return;
		}
		unsafe {
			s.item.as_mut_ptr().write(f());
		}
		s.has_init.store(true, Ordering::SeqCst);
	}

	/// Check to see if the [`OnceCell::call_once()`] function has set the inner
	/// value.
	pub fn is_completed(&self) -> bool {
		self.has_init.load(Ordering::Relaxed)
	}

	/// Wait for self.item to be set to something with a spinlock.
	pub fn wait(&self) -> &T {
		while !self.has_init.load(Ordering::Relaxed) {
			core::hint::spin_loop();
		}
		unsafe { &*self.item.as_ptr() }
	}
}
