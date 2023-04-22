use std::env;
use std::path::PathBuf;

const WHITELIST_FUNCTIONS: &[&str] = &[
	"battery_get_capacity",
	"battery_get_current",
	"battery_get_temperature",
	"battery_get_voltage",
	"competition_get_status",
	"controller_clear",
	"controller_clear_line",
	"controller_get_analog",
	"controller_get_battery_capacity",
	"controller_get_battery_level",
	"controller_get_digital",
	"controller_get_digital_new_press",
	"controller_is_connected",
	"controller_print",
	"controller_rumble",
	"controller_set_text",
	"distance_get",
	"distance_get_confidence",
	"distance_get_object_size",
	"distance_get_object_velocity",
	"ext_adi_analog_read",
	"ext_adi_digital_get_new_press",
	"ext_adi_digital_read",
	"ext_adi_digital_write",
	"ext_adi_led_clear_all",
	"ext_adi_led_clear_pixel",
	"ext_adi_led_init",
	"ext_adi_led_set",
	"ext_adi_led_set_all",
	"ext_adi_led_set_pixel",
	"ext_adi_pin_mode",
	"ext_adi_port_get_config",
	"ext_adi_port_get_value",
	"ext_adi_port_set_config",
	"ext_adi_port_set_value",
	"fdctl",
	"gps_get_accel",
	"gps_get_error",
	"gps_get_gyro_rate",
	"gps_get_heading",
	"gps_get_heading_raw",
	"gps_get_offset",
	"gps_get_rotation",
	"gps_get_status",
	"gps_initialize_full",
	"gps_set_data_rate",
	"gps_set_offset",
	"gps_set_position",
	"gps_set_rotation",
	"gps_tare_rotation",
	"imu_get_accel",
	"imu_get_euler",
	"imu_get_gyro_rate",
	"imu_get_heading",
	"imu_get_pitch",
	"imu_get_quaternion",
	"imu_get_roll",
	"imu_get_rotation",
	"imu_get_status",
	"imu_get_yaw",
	"imu_reset",
	"imu_set_euler",
	"imu_set_heading",
	"imu_set_pitch",
	"imu_set_roll",
	"imu_set_rotation",
	"imu_set_yaw",
	"imu_tare",
	"imu_tare_euler",
	"imu_tare_heading",
	"imu_tare_pitch",
	"imu_tare_roll",
	"imu_tare_rotation",
	"imu_tare_yaw",
	"lcd_clear",
	"lcd_clear_line",
	"lcd_initialize",
	"lcd_is_initialized",
	"lcd_print",
	"lcd_read_buttons",
	"lcd_set_text",
	"lcd_shutdown",
	"micros",
	"millis",
	"motor_get_actual_velocity",
	"motor_get_brake_mode",
	"motor_get_current_draw",
	"motor_get_current_limit",
	"motor_get_direction",
	"motor_get_efficiency",
	"motor_get_encoder_units",
	"motor_get_faults",
	"motor_get_flags",
	"motor_get_gearing",
	"motor_get_pos_pid",
	"motor_get_position",
	"motor_get_power",
	"motor_get_raw_position",
	"motor_get_target_position",
	"motor_get_target_velocity",
	"motor_get_temperature",
	"motor_get_torque",
	"motor_get_vel_pid",
	"motor_get_voltage",
	"motor_get_voltage_limit",
	"motor_get_zero_position_flag",
	"motor_is_over_current",
	"motor_is_over_temp",
	"motor_is_reversed",
	"motor_is_stopped",
	"motor_modify_profiled_velocity",
	"motor_move",
	"motor_move_absolute",
	"motor_move_relative",
	"motor_move_velocity",
	"motor_move_voltage",
	"motor_set_brake_mode",
	"motor_set_current_limit",
	"motor_set_encoder_units",
	"motor_set_gearing",
	"motor_set_reversed",
	"motor_set_voltage_limit",
	"motor_set_zero_position",
	"motor_tare_position",
	"mutex_create",
	"mutex_delete",
	"mutex_give",
	"mutex_take",
	"optical_disable_gesture",
	"optical_enable_gesture",
	"optical_get_brightness",
	"optical_get_gesture",
	"optical_get_gesture_raw",
	"optical_get_hue",
	"optical_get_led_pwm",
	"optical_get_proximity",
	"optical_get_raw",
	"optical_get_rgb",
	"optical_get_saturation",
	"optical_set_led_pwm",
	"registry_bind_port",
	"registry_get_bound_type",
	"registry_get_plugged_type",
	"registry_unbind_port",
	"rotation_get_angle",
	"rotation_get_position",
	"rotation_get_reversed",
	"rotation_get_velocity",
	"rotation_reset",
	"rotation_reset_position",
	"rotation_reverse",
	"rotation_set_data_rate",
	"rotation_set_position",
	"rotation_set_reversed",
	"serctl",
	"task_create",
	"task_delay",
	"task_delay_until",
	"task_delete",
	"task_get_by_name",
	"task_get_count",
	"task_get_current",
	"task_get_name",
	"task_get_priority",
	"task_get_state",
	"task_join",
	"task_notify",
	"task_notify_clear",
	"task_notify_ext",
	"task_notify_take",
	"task_resume",
	"task_set_priority",
	"task_suspend",
	"usd_is_installed",
	"vision_clear_led",
	"vision_create_color_code",
	"vision_get_by_code",
	"vision_get_by_sig",
	"vision_get_by_size",
	"vision_get_exposure",
	"vision_get_object_count",
	"vision_get_signature",
	"vision_get_white_balance",
	"vision_print_signature",
	"vision_read_by_code",
	"vision_read_by_sig",
	"vision_read_by_size",
	"vision_set_auto_white_balance",
	"vision_set_exposure",
	"vision_set_led",
	"vision_set_signature",
	"vision_set_white_balance",
	"vision_set_wifi_mode",
	"vision_set_zero_point",
	"vision_signature_from_utility",
];
const WHITELIST_TYPES: &[&str] = &["ext_adi_led_t"];
const WHITELIST_VARS: &[&str] = &["errno"];

const BLACKLIST_ITEMS: &[&str] = &["vision_object_s_t"];
const BITFIELD_ENUM: &[&str] = &[];

fn main() {
	println!("cargo:rerun-if-changed=build.rs");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	let kernel_path = out_path.join("pros");

	std::fs::create_dir_all(kernel_path.clone()).expect("failed to create extract dir");

	// Extract kernel zip
	let kernel_file =
		std::fs::File::open("kernel@3.8.0.zip").expect("failed to open kernel zip file");
	zip::read::ZipArchive::new(kernel_file)
		.unwrap()
		.extract(kernel_path.clone())
		.expect("failed to extract zip");

	// Add link search paths for the firmware and link
	println!(
		"cargo:rustc-link-search={}",
		kernel_path.clone().join("firmware").display()
	);

	// Get std headers for c to generate bindings correctly
	let command = std::process::Command::new("arm-none-eabi-gcc")
		.args(&["-E", "-Wp,-v", "-xc", "/dev/null"])
		.output()
		.expect("installed arm-none-eabi noob");

	let mut include_paths = vec![];
	let mut in_lines = false;

	let stderr = std::str::from_utf8(&command.stderr).unwrap();
	for err in stderr.lines() {
		if err == "#include <...> search starts here:" {
			in_lines = true;
		} else if err == "End of search list." {
			in_lines = false;
		}

		if in_lines {
			include_paths.push(format!("-I{}", err.trim()))
		}
	}
	include_paths.push(format!(
		"-I{}",
		kernel_path.clone().join("include").to_string_lossy()
	));

	// Generate bindings
	let mut bindings = bindgen::Builder::default()
		.header(kernel_path.join("include/api.h").to_string_lossy())
		.header(kernel_path.join("include/pros/apix.h").to_string_lossy())
		.clang_args(&["-target", "arm-none-eabi"])
		.clang_args(include_paths)
		.ctypes_prefix("libc")
		.layout_tests(false)
		.generate_comments(false)
		.use_core();

	for func in WHITELIST_FUNCTIONS {
		bindings = bindings.allowlist_function(func);
	}
	for ty in WHITELIST_TYPES {
		bindings = bindings.allowlist_type(ty);
	}
	for var in WHITELIST_VARS {
		bindings = bindings.allowlist_var(var);
	}

	for item in BLACKLIST_ITEMS {
		bindings = bindings.blocklist_item(item);
	}
	for bitfield in BITFIELD_ENUM {
		bindings = bindings.allowlist_function(bitfield);
	}

	bindings
		.generate()
		.expect("unabled to generate bindings")
		.write_to_file(out_path.join("bindings.rs"))
		.expect("failed to write bindings");
}
