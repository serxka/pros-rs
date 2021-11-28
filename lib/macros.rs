#[macro_export]
macro_rules! robot {
	($robot:tt) => {
		// WHAT THE ACTUAL FUCK WHY WON'T TY WORK
		static mut ROBOT: $crate::util::StaticMut<$robot> = $crate::util::StaticMut::new();

		#[no_mangle]
		extern "C" fn initialize() {
			$crate::rtos::spawn(|| {
				ROBOT.call_once(|| $robot::new());
			});
		}

		#[no_mangle]
		extern "C" fn disabled() {
			ROBOT.wait().disabled()
		}

		#[no_mangle]
		extern "C" fn competition_initialize() {
			ROBOT.wait().competition_init()
		}

		#[no_mangle]
		extern "C" fn autonomous() {
			ROBOT.wait().autonomous()
		}

		#[no_mangle]
		extern "C" fn opcontrol() {
			ROBOT.wait().opcontrol()
		}
	};
}
