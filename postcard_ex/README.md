# Postcard example

This is a simple example on how to use `postcard` to create wire commands.
The idea behind the example is the following:

- Communicate between host and device.
- The host sends commands to the device and, if it is a query, the device responds with the requested information.
- All commands and arguments structs are defined in a `commands.rs` file, which can be shared between host and device.
- Every commands is an enum variant that hold arguments or not, depending if necessary.
- The `Commands::Unknown` variant is used to handle unknown commands, user error or transfer problem.

The `main.rs` file contains both, the host and the device.
The host basically will do what is given in `main()`,
while the device will receive the commands and process them,
similar to `decode()`.
