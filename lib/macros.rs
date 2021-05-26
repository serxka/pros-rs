#[macro_export]
macro_rules! robot {
	($robot:ty) => {
		// Somehow have a static $robot here and call into all the functions

		#[no_mangle]
		extern "C" fn initialize() {
			$crate::println!("initialize()");
		}

		#[no_mangle]
		extern "C" fn disabled() {}

		#[no_mangle]
		extern "C" fn competition_initialize() {}

		#[no_mangle]
		extern "C" fn autonomous() {}

		#[no_mangle]
		extern "C" fn opcontrol() {}
	};
}
