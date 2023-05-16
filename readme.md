# pros-rs
rust bindings and environment for the vex pros open source kernel, very much a working in progress at the moment

## Tools Required
- arm-none-eabi gcc and binutils
- prosv5 cli tool
- rust armv7a-none-eabi target
- rustc-src component

**Commands to get Rust part working**, also just follow what ever else it tells you to install.
```
$ rustup target install armv7a-none-eabi
...
$ rustup component add rust-src
...
```
