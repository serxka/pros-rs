# pros-rs
Rust bindings and environment for the vex pros open source kernel, very much a working in progress at the moment

## Getting Started
After acquiring all the proper software in [Tools Required](#tools-required)
To be able to use this crate properly you will need to create a new project from the `pros-rs-template`. This can be done using cargo generate with: `cargo generate --git https://github.com/serxka/pros-rs-template.git`

## Tools Required
It is already assumed that rustup is installed, if not please visit [rustup.rs](https://rustup.rs/). Tools required:
+ GCC and binutils for the `arm-none-eabi` target
+ libclang
+ `prosv5` CLI tool
+ `armv7a-none-eabi` target & `rustc-src` component in rustup

### Commands for quick setup.
**Commands to install Rust components**
```sh
$ rustup target install armv7a-none-eabi
...
$ rustup component add rust-src
...
```

**Command to install PROS CLI**
```
$ pip install git+https://github.com/purduesigbots/pros-cli.git
...
```

**Installation for `arm-none-eabi` GCC/binutills**. This can either be installed from the users distros repositories or it can be [downloaded](https://developer.arm.com/downloads/-/gnu-rm) and added to the PATH.

**Installation for `libclang`**. This should be installed from the users distros repositories. This is often packages with the `clang` package or separate as `libclang-dev` in distributions such as Debian.
