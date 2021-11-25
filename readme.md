# pros-rs
rust bindings and environment for the vex pros open source kernel, very much a working in progress at the movement

## Priorities
Priorities for bindings and getting things working
* FreeRTOS (tasks, mutexs, atomics, etc)
* Controller / Motors
* ADI (TriPort)
* Proper cstring interop
* All other bindings
* Testing

## Tools Required
* arm-none-eabi gcc and binutils
* prosv5 cli tool
* rust armv7a-none-eabi target
* rustc-src component

**Commands to get Rust part working**, also just follow what ever else it tells you to install.
```
$ rustup target install armv7a-none-eabi
...
$ rustup component add rust-src
...
```
