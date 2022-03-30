/// Used to create a new robot entry point.
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
		static mut ROBOT: $crate::rtos::StaticMut<$robot> = $crate::rtos::StaticMut::new();

		#[no_mangle]
		extern "C" fn initialize() {
			$crate::rtos::spawn(|| unsafe {
				let devices = $crate::devices::Devices::new();
				ROBOT.call_once(|| $robot::new(devices));
			});
		}

		#[no_mangle]
		extern "C" fn disabled() {
			unsafe { ROBOT.wait().disabled() }
		}

		#[no_mangle]
		extern "C" fn competition_initialize() {
			unsafe { ROBOT.wait().competition_init() }
		}

		#[no_mangle]
		extern "C" fn autonomous() {
			unsafe { ROBOT.wait().autonomous() }
		}

		#[no_mangle]
		extern "C" fn opcontrol() {
			unsafe { ROBOT.wait().opcontrol() }
		}
	};
}

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
