//! Functionally relevant to PROS RTOS multitasking, also contains time keeping
//! and synchronisation primitives.

pub mod action;
pub mod tasks;
pub mod time;

use crate::bindings;

use core::{
	cell::UnsafeCell,
	mem::MaybeUninit,
	ops::{Deref, DerefMut},
	sync::atomic::{AtomicBool, Ordering},
};

struct MutexInner {
	ptr: *mut libc::c_void,
}

impl MutexInner {
	pub fn new() -> MutexInner {
		let ptr = unsafe { bindings::mutex_create() };
		if ptr == core::ptr::null_mut() {
			panic!("failed to create mutex");
		}
		MutexInner { ptr }
	}

	pub fn take(&self, timeout: u32) -> bool {
		unsafe { bindings::mutex_take(self.ptr, timeout) }
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
	pub fn lock(&self) -> Option<MutexGuard<'_, T>> {
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
	/// This function will return an option if the Mutex was unable to be
	/// obtained due to a timeout.
	pub fn lock_timeout(&self, timeout: u32) -> Option<MutexGuard<'_, T>> {
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
