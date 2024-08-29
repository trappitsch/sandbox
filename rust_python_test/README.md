# rust-python-test

Simple sandbox project to check out `PyO3` and `maturin`.
The idea here is to use this for the `rimseval` project.
The super slow thing at the moment is to convert `lst` files to `crd` files.
However, this could easily be done in Rust.
Here is a simple example on how to interface between `python` and `rust`
with this kind of capability.

## Run:

Editing the `lib.rs` file requires compilation.
This can be achieved with `maturin develop`.
We want to skip the install - `rye` takes care of this.
The sandbox function to call the Rust code is included as a `rye` script.
So the next two commands will compile and run this project.

```bash
maturin develop --skip-install
rye run l2c
```

* License: MIT
