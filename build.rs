use std::env;
use std::path::PathBuf;

const WHITELIST_FUNCTIONS: &[&str] = &[];
const WHITELIST_TYPES: &[&str] = &[];
const WHITELIST_VARS: &[&str] = &[];
const BITFIELD_ENUM: &[&str] = &[];

fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	
	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	let kernel_path = out_path.join("pros");
	
	std::fs::create_dir_all(kernel_path.clone()).expect("failed to create extract dir");
	
	// Extract kernel zip
	let kernel_file = std::fs::File::open("kernel@3.4.0.zip").expect("failed to open kernel zip file");
	zip::read::ZipArchive::new(kernel_file).unwrap().extract(kernel_path.clone()).expect("failed to extract zip");

	// Get std headers for c to generate bindings correctly
	let command = std::process::Command::new("arm-none-eabi-gcc")
		.args(&["-E", "-Wp,-v", "-xc", "/dev/null"])
        	.output()
        	.expect("installed arm-none-eabi noob");
        
        let mut include_paths = vec![];
        let mut in_lines = false;
        
        let stderr = std::str::from_utf8(&command.stderr).unwrap();
        for err in stderr.lines() {
		if err == "#include <...> search starts here:" { in_lines = true; }
		else if err == "End of search list." { in_lines = false; }
		
		if in_lines {
			include_paths.push(format!("-I{}", err.trim()))
		}
        }
        include_paths.push(format!("-I{}", kernel_path.clone().join("include").to_string_lossy()));

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