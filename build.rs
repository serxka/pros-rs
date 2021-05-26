use std::env;
use std::path::PathBuf;

const WHITELIST_FUNCTIONS: &[&str] = &[
    "adi_analog_calibrate",
    "adi_analog_read",
    "adi_analog_read_calibrated",
    "adi_analog_read_calibrated_HR",
    "adi_digital_get_new_press",
    "adi_digital_read",
    "adi_digital_write",
    "adi_encoder_get",
    "adi_encoder_init",
    "adi_encoder_reset",
    "adi_encoder_shutdown",
    "adi_motor_get",
    "adi_motor_set",
    "adi_motor_stop",
    "adi_pin_mode",
    "adi_port_get_config",
    "adi_port_get_value",
    "adi_port_set_config",
    "adi_port_set_value",
    "adi_ultrasonic_get",
    "adi_ultrasonic_init",
    "adi_ultrasonic_shutdown",
    "adi_gyro_init",
    "adi_gyro_get",
    "adi_gyro_reset",
    "adi_gyro_shutdown",
    "ext_adi_analog_calibrate",
    "ext_adi_analog_read",
    "ext_adi_analog_read_calibrated",
    "ext_adi_analog_read_calibrated_HR",
    "ext_adi_digital_get_new_press",
    "ext_adi_digital_read",
    "ext_adi_digital_write",
    "ext_adi_encoder_get",
    "ext_adi_encoder_init",
    "ext_adi_encoder_reset",
    "ext_adi_encoder_shutdown",
    "ext_adi_motor_get",
    "ext_adi_motor_set",
    "ext_adi_motor_stop",
    "ext_adi_pin_mode",
    "ext_adi_port_get_config",
    "ext_adi_port_get_value",
    "ext_adi_port_set_config",
    "ext_adi_port_set_value",
    "ext_adi_ultrasonic_get",
    "ext_adi_ultrasonic_init",
    "ext_adi_ultrasonic_shutdown",
    "ext_adi_gyro_init",
    "ext_adi_gyro_get",
    "ext_adi_gyro_reset",
    "ext_adi_gyro_shutdown",
    "distance_get",
    "distance_get_confidence",
    "distance_get_object_size",
    "distance_get_object_velocity",
    "imu_reset",
    "imu_get_rotation",
    "imu_get_heading",
    "imu_get_quaternion",
    "imu_get_euler",
    "imu_get_pitch",
    "imu_get_roll",
    "imu_get_yaw",
    "imu_get_gyro_rate",
    "imu_get_accel",
    "imu_get_status",
    "imu_tare_heading",
    "imu_tare_rotation",
    "imu_tare_pitch",
    "imu_tare_roll",
    "imu_tare_yaw",
    "imu_tare_euler",
    "imu_tare",
    "imu_set_euler",
    "imu_set_rotation",
    "imu_set_heading",
    "imu_set_pitch",
    "imu_set_roll",
    "imu_set_yaw",
    "lcd_clear",
    "lcd_clear_line",
    "lcd_initialize",
    "lcd_is_initialized",
    "lcd_print",
    "lcd_read_buttons",
    "lcd_register_btn0_cb",
    "lcd_register_btn1_cb",
    "lcd_register_btn2_cb",
    "lcd_set_text",
    "lcd_shutdown",
    "battery_get_capacity",
    "battery_get_current",
    "battery_get_temperature",
    "battery_get_voltage",
    "competition_get_status",
    "competition_is_autonomous",
    "competition_is_connected",
    "competition_is_disabled",
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
    "usd_is_installed",
    "motor_move",
    "motor_move_absolute",
    "motor_move_relative",
    "motor_move_velocity",
    "motor_move_voltage",
    "motor_modify_profiled_velocity",
    "motor_get_target_position",
    "motor_get_target_velocity",
    "motor_get_actual_velocity",
    "motor_get_current_draw",
    "motor_get_direction",
    "motor_get_efficiency",
    "motor_get_faults",
    "motor_get_flags",
    "motor_get_position",
    "motor_get_power",
    "motor_get_raw_position",
    "motor_get_temperature",
    "motor_get_torque",
    "motor_get_voltage",
    "motor_get_zero_position_flag",
    "motor_is_stopped",
    "motor_is_over_current",
    "motor_is_over_temp",
    "motor_convert_pid",
    "motor_convert_pid_full",
    "motor_get_brake_mode",
    "motor_get_current_limit",
    "motor_get_encoder_units",
    "motor_get_gearing",
    "motor_get_pos_pid",
    "motor_get_vel_pid",
    "motor_get_voltage_limit",
    "motor_is_reversed",
    "motor_set_brake_mode",
    "motor_set_current_limit",
    "motor_set_encoder_units",
    "motor_set_gearing",
    "motor_set_pos_pid",
    "motor_set_pos_pid_full",
    "motor_set_reversed",
    "motor_set_vel_pid",
    "motor_set_vel_pid_full",
    "motor_set_voltage_limit",
    "motor_set_zero_position",
    "motor_tare_position",
    "optical_get_hue",
    "optical_get_saturation",
    "optical_get_brightness",
    "optical_get_proximity",
    "optical_set_led_pwm",
    "optical_get_led_pwm",
    "optical_get_rgb",
    "optical_get_raw",
    "optical_get_gesture",
    "optical_get_gesture_raw",
    "optical_enable_gesture",
    "optical_disable_gesture",
    "rotation_reset",
    "rotation_set_position",
    "rotation_reset_position",
    "rotation_get_position",
    "rotation_get_velocity",
    "rotation_get_angle",
    "rotation_set_reversed",
    "rotation_reverse",
    "rotation_get_reversed",
    "delay",
    "millis",
    "micros",
    "mutex_create",
    "mutex_delete",
    "mutex_give",
    "mutex_take",
    "task_create",
    "task_delay",
    "task_delay_until",
    "task_delete",
    "task_get_by_name",
    "task_get_count",
    "task_get_name",
    "task_get_priority",
    "task_get_state",
    "task_notify",
    "task_notify_clear",
    "task_notify_ext",
    "task_notify_take",
    "task_resume",
    "task_set_priority",
    "task_suspend",
    "vision_clear_led",
    "vision_signature_from_utility",
    "vision_create_color_code",
    "vision_get_by_sig",
    "vision_get_by_size",
    "vision_get_by_code",
    "vision_get_exposure",
    "vision_get_object_count",
    "vision_get_signature",
    "vision_get_white_balance",
    "vision_print_signature",
    "vision_read_by_sig",
    "vision_read_by_size",
    "vision_read_by_code",
    "vision_set_auto_white_balance",
    "vision_set_exposure",
    "vision_set_led",
    "vision_set_signature",
    "vision_set_white_balance",
    "vision_set_zero_point",
    "vision_set_wifi_mode",
];
const WHITELIST_TYPES: &[&str] = &[];
const WHITELIST_VARS: &[&str] = &[];
const BITFIELD_ENUM: &[&str] = &[];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let kernel_path = out_path.join("pros");

    std::fs::create_dir_all(kernel_path.clone()).expect("failed to create extract dir");

    // Extract kernel zip
    let kernel_file =
        std::fs::File::open("kernel@3.4.0.zip").expect("failed to open kernel zip file");
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
        .clang_args(&["-target", "arm-none-eabi"])
        .clang_args(include_paths)
        .ctypes_prefix("libc")
        .layout_tests(false)
        .use_core();

    for func in WHITELIST_FUNCTIONS {
        bindings = bindings.allowlist_function(func);
    }
    for ty in WHITELIST_TYPES {
        bindings = bindings.allowlist_function(ty);
    }
    for var in WHITELIST_VARS {
        bindings = bindings.allowlist_function(var);
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
