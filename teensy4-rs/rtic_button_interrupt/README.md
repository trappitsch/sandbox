# Blinky

The embedded world Hello World program,
here implemented with two LEDs, 
and an [RTIC](https://rtic.rs/2/book/en/preface.html) framework.

The starter code is the template following the cargo-generate
instructions on the teensy4-rs repository. However, I replaced
the clock source code for the monotonic from the 
rtic example repository. This clock seems to run a lot better in 
sync than the other one. 

Furthermore, I configured some more pins, see the common pinout 
table for which pin is associated with which gpio port.

## Compile, copy, and upload

There's a `justfile` provided to compile, object copy, and upload
the hex code. 
This follows instructions on the `teensy4-rs` repo.
To do everything, just type `just`.

