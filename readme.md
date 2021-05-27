# pros-rs
rust bindings and enviroment for the vex pros open source kernel, very much a working in progress at the momement

## Priorities
Priorities for bindings and gettings things working
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
* rustc-src componenet

**Commands to get Rust part working**
```
$ rustup target install armv7a-none-eabi
...
$ rustup component add rust-src
...
```
