# pros-rs
rust bindings and environment for the vex pros open source kernel, very much a working in progress at the moment

## Priorities
Priorities for bindings and getting things working
- [x] FreeRTOS
	- [x] Tasks
	- [x] Atomics
	- [x] Mutexes
- [x] Controllers
- [x] Motors (documentation needed)
- [ ] Sensors
	- [x] Rotation sensor (documentation needed)
	- [x] IMU
	- [ ] Distance sensor
	- [ ] Optical sensor
	- [ ] Vision sensor
	- [ ] GPS sensor
- [x] ADI (TriPort)
- [ ] Proper low alloc cstring interop
- [ ] Testing
- [ ] Display, GUI (home rolled immediate mode?)

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
