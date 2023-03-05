/// Used to create a new robot entry point. Your program will not link without
/// this, it is **required**.
///
/// # Examples
/// ```
/// struct MyRobot;
/// impl Robot for MyRobot {
/// 	...
/// }
///
/// robot!(MyRobot);
/// ```
#[macro_export]
macro_rules! robot {
	($robot:tt) => {
		#[doc(hidden)]
		static ROBOT: $crate::rtos::OnceCell<($robot, $crate::rtos::tasks::CompetitionState)> =
			$crate::rtos::OnceCell::new();

		#[doc(hidden)]
		#[no_mangle]
		extern "C" fn initialize() {
			use $crate::{devices, rtos::tasks, Robot};

			$crate::rtos::tasks::spawn(|| unsafe {
				ROBOT.call_once(|| {
					let devices = devices::Devices::new();
					let state = tasks::CompetitionState::new();
					($robot::new(devices), state)
				});
			});
		}

		#[doc(hidden)]
		#[no_mangle]
		extern "C" fn disabled() {
			use $crate::{rtos::tasks, Robot};

			let robot = ROBOT.wait();
			robot.0.disabled(robot.1.clone());
		}

		#[doc(hidden)]
		#[no_mangle]
		extern "C" fn competition_initialize() {
			use $crate::{rtos::tasks, Robot};

			let robot = ROBOT.wait();
			robot.0.competition_init(robot.1.clone());
		}

		#[doc(hidden)]
		#[no_mangle]
		extern "C" fn autonomous() {
			use $crate::{rtos::tasks, Robot};
			let robot = ROBOT.wait();
			robot.1.add_autonomous(tasks::Task::current());

			tasks::spawn(|| {
				robot.0.autonomous(robot.1.clone());
			})
			.join();
		}

		#[doc(hidden)]
		#[no_mangle]
		extern "C" fn opcontrol() {
			use $crate::{rtos::tasks, Robot};
			let robot = ROBOT.wait();
			robot.1.add_opcontrol(tasks::Task::current());

			tasks::spawn(|| {
				robot.0.opcontrol(robot.1.clone());
			})
			.join();
		}
	};
}

#[doc(hidden)]
pub use pros_macro::action_internal;

/// Poll actions until one is found to be complete, if none are complete, then
/// it will sleep until the next time that one is suggested to be available.
///
/// # Examples
/// It is common to use this in `opcontrol` to understand when the task should
/// exit and allow autonomous to run.
/// ```
/// fn opcontrol(&'static self, state: CompetitionState) {
/// 	let mut timer = Interval::new(Duration::from_millis(20));
///
/// 	loop {
/// 		// main opcontrol code
/// 		action! {
/// 			_ = state.task_done(CompetitionTask::OpControl) => break,
/// 			_ = timer.action() => continue
/// 		}
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! action {
	($($tokens:tt)*) => {{
		$crate::macros::action_internal! {
			$( $tokens )*
		}
	}}
}

#[doc(hidden)]
#[macro_export]
macro_rules! pros_unsafe_err {
	($fn:ident, err = $err:expr) => {
		pros_unsafe_err!($fn, err = $err,)
	};
	($fn:ident, err = $err:expr, $($x:expr),*) => {
		match unsafe { $fn ( $($x,)* ) } {
			$crate::util::PROS_ERR => Err($err),
			x => Ok(x)
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! pros_unsafe_err_bool {
	($fn:ident, err = $err:expr) => {
		pros_unsafe_err_bool!($fn, err = $err,)
	};
	($fn:ident, err = $err:expr, $($x:expr),*) => {
		match unsafe { $fn ( $($x,)* ) } {
			$crate::util::PROS_ERR => Err($err),
			1 => Ok(true),
			0 => Ok(false),
			_ => unreachable!(),
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! pros_unsafe_err_u32 {
	($fn:ident, err = $err:expr) => {
		pros_unsafe_err_u32!($fn, err = $err,)
	};
	($fn:ident, err = $err:expr, $($x:expr),*) => {
		match unsafe { $fn ( $($x,)* ) } {
			$crate::util::PROS_ERR_U32 => Err($err),
			x => Ok(x)
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! pros_unsafe_err_f {
	($fn:ident, err = $err:expr) => {
		pros_unsafe_err_f!($fn, err = $err,)
	};
	($fn:ident, err = $err:expr, $($x:expr),*) => {
		{
			let res = unsafe { $fn ( $($x,)* ) };
			if res == $crate::util::PROS_ERR_F {
				Err($err)
			} else {
				Ok(res)
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! pros_unsafe_err_sig {
	($fn:ident, err = $err:expr) => {
		pros_unsafe_err_sig!($fn, err = $err,)
	};
	($fn:ident, err = $err:expr, $($x:expr),*) => {
		match unsafe { $fn ( $($x,)* ) } {
			x if x.id == $crate::util::PROS_ERR_VISION_OBJECT_SIG => Err($err),
			x => Ok(x)
		}
	};
}
