# Doctor
Checks if all links mentioned in cargo docs of a crate are active 

[![Build Status](https://travis-ci.org/Dylan-DPC/doctor.svg?branch=master)](https://travis-ci.org/Dylan-DPC/doctor) 
[![Latest Version](https://img.shields.io/crates/v/cargo-doctor.svg)](https://crates.io/crates/cargo-doctor) 

# Installation

```bash
$ cargo install cargo-doctor
```

You will need nightly to run this crate as of 0.1.2. 

(Please check cargo's documentation to learn how cargo install works and how to set up your system so it finds binaries installed by cargo.)



# Usage: 

Doctor scans the docs and checks if every link mentioned is live or broken. It can work with both docs generated locally using `cargo doc` and those uploaded on `docs.rs`. 

###Local Docs:
To run doctor on the local docs use the `-l` flag: 
 ```bash
 cargo doctor -l
 # or
 cargo doctor --local
 ```
 
 This will output either a success message or the list of links that are not found/active. 
 
 Ensure you have run `cargo doc` before running doctor. 
 
 ### Remote Docs: 
 
 To run doctor on docs.rs links of your crate, run: 
```bash
cargo doctor -r 
# or
cargo doctor --remote
```

It will generate the link from the crate name and version provided in `Cargo.toml` of your crate.

You can also use `--path` or `-p` to specify a different path along with `-r` or `-l`

```bash
cargo doctor -r --path="https://docs.rs/cargo_doctor/0.1.0/cargo_doctor/"
```
(Note: the path should end with `/`)

# Contribution:

If you want to suggest any new feature or report a bug, you can open an issue here or drop in a pull request directly. 

Right now, I still need to tests for most of the functions, so you can test it locally by running: 

```bash 
cargo run -- -r
```

This package is written using Rust 1.30.0-nightly.  

When submitting a Pull request, run `cargo fmt` on the latest nightly before committing. 

# License

Licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.