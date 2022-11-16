use crate::bindings;
use crate::rtos::{tasks::Task, time::Duration};

/// Budget future go brrrp
pub trait Action {
	/// The type our action will eventually resolve too.
	type Output;

	/// Poll to see if our action has been completed. Returns straight away.
	fn poll(&mut self) -> Poll<Self::Output>;
	/// Soonest possible time our action might be complete.
	fn next(&self) -> NextSleep;
}

/// An enum to store whether our action is complete or still waiting.
pub enum Poll<T> {
	/// Our action was complete, contains the final value the action resolved
	/// too.
	Complete(T),
	/// Our action has not been completed.
	Waiting,
}

impl<T> Poll<T> {
	pub fn is_complete(&self) -> bool {
		match self {
			Poll::Complete(_) => true,
			_ => false,
		}
	}
}

/// When we should sleep till next checking our action.
pub enum NextSleep {
	/// We have no idea when our action will be ready, sleeping would be
	/// inappropriate. Do other tasks or yield our time-slice.
	Never,
	/// The executor should wait until a notification is sent to this task. If
	/// `None` is used as our timeout wait indefinitely.
	Notification(Option<Duration>),
	/// The executor should sleep until for this length of time and the poll
	/// again to see if the action is complete.
	Timestamp(Duration),
}

impl NextSleep {
	pub fn sleep(self) {
		match self {
			NextSleep::Never => (),
			NextSleep::Notification(time) => {
				let time = time.map(|x| x.as_millis() as u32).unwrap_or(u32::MAX);
				unsafe {
					bindings::task_notify_take(true, time);
				}
			}
			NextSleep::Timestamp(time) => Task::delay(time),
		}
	}
}
