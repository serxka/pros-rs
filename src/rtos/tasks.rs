use crate::bindings;
use crate::rtos::{
	action::{Action, NextSleep, Poll},
	Mutex,
};
use crate::util::to_cstring;

use alloc::{boxed::Box, string::String, sync::Arc};
use core::time::Duration;

#[derive(Clone)]
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

	/// Return an handle to the current task.
	pub fn current() -> Task {
		unsafe {
			let repr = bindings::task_get_current();
			Task { repr, name: None }
		}
	}

	/// Delay the current task for at least however many milliseconds that is
	/// stored in the duration.
	///
	/// # Examples
	/// ```
	/// // Delay the current task from execution for at least 500ms.
	/// delay(Duration::from_millis(500));
	/// // Delay the current task from execution for about 4 seconds.
	/// delay(Duration::from_seconds(4));
	/// ```
	pub fn delay(dur: Duration) {
		unsafe { bindings::task_delay(dur.as_millis() as u32) }
	}

	/// Get the name of this thread, it is possible that this thread does not
	/// have name. In this case the string returned will be of zero length.
	pub fn name<'a>(&mut self) -> &'a str {
		if self.name.is_none() {
			self.name = Some(unsafe { bindings::task_get_name(self.repr) as _ });
		}

		let name = self.name.unwrap();
		unsafe {
			let slice = core::slice::from_raw_parts(name, libc::strlen(name)) as &[u8];
			core::str::from_utf8(slice).unwrap()
		}
	}

	/// If this task was previously suspended before it will now considered
	/// eligible for execution by the RTOS scheduler. This function has no
	/// effect if the task was not marked as suspended. This does **not**
	/// necessarily tell the scheduler to run this task immediately.
	#[doc(alias = "unpark")]
	pub fn resume(&self) {
		unsafe {
			bindings::task_resume(self.repr);
		}
	}

	/// Suspend this task from being run by the RTOS scheduler. This task will
	/// not take CPU time unless it is later called with [`Task::resume()`].
	#[doc(alias = "park")]
	pub fn suspend(&self) {
		unsafe {
			bindings::task_suspend(self.repr);
		}
	}

	/// Block this tasks execution until this task has completed and exited.
	///
	/// # Examples
	/// This code will **always** print in the order "A B C".
	/// ```
	/// print!("A ");
	/// tasks::spawn(|| {
	/// 	print!("B ");
	/// })
	/// .join();
	/// println!("C");
	/// ```
	pub fn join(&self) {
		// Nothing happens when we try to join ourselves, but it's important to put an
		// assert to check we aren't accidentally.
		assert!(self.repr != Task::current().repr);
		unsafe {
			bindings::task_join(self.repr);
		}
	}

	/// Remove as task from RTOS task management. The task being deleted will be
	/// removed from all queues. Memory and resource allocated by this task will
	/// not be freed.
	///
	/// It is not advisable to use this function. Instead prefer using
	/// [`Task::join()`], letting the calling task cleanly exit, calling `Drop`
	/// as needed. If this is called on the current task then it will be
	/// deleted.
	///
	/// # Examples
	/// ```
	/// let task = tasks::spawn(|| loop {});
	/// // oh no! our task is stuck, lets delete it
	/// task.delete();
	/// ```
	pub fn delete(self) {
		unsafe {
			bindings::task_delete(self.repr);
		}
	}

	pub fn get_state(&self) -> TaskState {
		unsafe { bindings::task_get_state(self.repr).into() }
	}
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

/// What state the task is in currently as seen by FreeRTOS.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
	/// This task is currently running and is actively using CPU time.
	Running,
	/// This task is not actively running, but it ready to be scheduled and ran
	/// at anytime.
	Ready,
	/// This task is delayed, blocked by a mutex or semaphore, or is waiting on
	/// an I/O operation.
	Blocked,
	/// This task has been suspended using [`Task::suspend()`]
	Suspended,
	/// This task has finished, its resources are gone and will not ever be
	/// scheduled again.
	Finished,
	/// The state for this task is unknown, it does not appear to be a known
	/// previous or current task.
	Invalid,
}

impl From<bindings::task_state_e_t> for TaskState {
	fn from(f: bindings::task_state_e_t) -> Self {
		use bindings::*;
		#[allow(non_upper_case_globals)]
		match f {
			task_state_e_t_E_TASK_STATE_RUNNING => Self::Running,
			task_state_e_t_E_TASK_STATE_READY => Self::Ready,
			task_state_e_t_E_TASK_STATE_BLOCKED => Self::Blocked,
			task_state_e_t_E_TASK_STATE_SUSPENDED => Self::Suspended,
			task_state_e_t_E_TASK_STATE_DELETED => Self::Finished,
			task_state_e_t_E_TASK_STATE_INVALID => Self::Invalid,
			_ => panic!("unknown task state: ({})", f),
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
				name.as_ptr() as _,
			);
			if res == core::ptr::null_mut() {
				_ = Box::from_raw(arg); // rebox pointer to avoid leak if failed to create task
				Err(()) // TODO: error handling
				 // The only possible error is failure to allocate memory, this
				 // is a pretty fatal death in Rust and to try and handle it
				 // from this point is somewhat redundant.
			} else {
				Ok(Task {
					repr: res,
					name: None,
				})
			}
		}
	}
}

/// Spawn a new task with the default stack size and priority.
///
/// # Panics
/// Panics if the task cannot be spawned, refer to [`TaskBuilder::spawn()`]'s
/// error return type for more possible reasons.
///
/// # Examples
/// ```
/// // Every 200ms print another message to the serial console.
/// spawn(|| {
/// 	let mut i = 1;
/// 	loop {
/// 		println!("hello again for... it's be {} times hasn't it?", i);
/// 		Task::delay(Duration::from_millis(200));
/// 	}
/// })
/// ```
pub fn spawn<F: FnOnce() + Send + 'static>(f: F) -> Task {
	TaskBuilder::new().spawn(f).expect("failed to spawn task")
}

#[derive(Default)]
struct CompetitionStateInner {
	opcontrol_task: Option<Task>,
	autonomous_task: Option<Task>,
}

pub struct CompetitionState(Arc<Mutex<CompetitionStateInner>>);

impl CompetitionState {
	#[doc(hidden)]
	pub fn new() -> Self {
		Self(Arc::new(Mutex::new(Default::default())))
	}

	#[doc(hidden)]
	pub fn add_opcontrol(&self, task: Task) {
		let mut s = self.0.lock();
		s.opcontrol_task = Some(task);
	}

	#[doc(hidden)]
	pub fn add_autonomous(&self, task: Task) {
		let mut s = self.0.lock();
		s.autonomous_task = Some(task);
	}

	#[doc(hidden)]
	pub fn clone(&self) -> Self {
		Self(self.0.clone())
	}

	/// Check to see if the specific competition task has been completed or
	/// killed by the Field Management System.
	pub fn task_done(&'_ self, task: CompetitionTask) -> impl Action + '_ {
		struct TaskDoneAction<'a>(&'a CompetitionState, CompetitionTask);

		impl<'a> Action for TaskDoneAction<'a> {
			type Output = ();

			fn poll(&mut self) -> Poll<Self::Output> {
				fn check_done(task: &Task) -> bool {
					match task.get_state() {
						TaskState::Suspended | TaskState::Finished | TaskState::Invalid => true,
						_ => false,
					}
				}

				let inner = self.0 .0.lock();
				let done = match self.1 {
					CompetitionTask::OpControl => {
						check_done(inner.opcontrol_task.as_ref().expect(
							"cannot get state of opcontrol task when it has not yet been run",
						))
					}
					CompetitionTask::Autonomous => {
						check_done(inner.autonomous_task.as_ref().expect(
							"cannot get state of autonomous task when it has not yet been run",
						))
					}
				};
				if done {
					Poll::Complete(())
				} else {
					Poll::Waiting
				}
			}

			fn next(&mut self) -> NextSleep {
				NextSleep::Never
			}
		}

		TaskDoneAction(self, task)
	}
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CompetitionTask {
	OpControl,
	Autonomous,
}
