# Contributing

- [Contributing](#contributing)
  - [Introduction](#introduction)
  - [Quality control](#quality-control)
  - [Visual Studio Code](#visual-studio-code)
  - [Readers and Writers](#readers-and-writers)
  - [Sources and Sinks](#sources-and-sinks)
  - [Buffering](#buffering)
  - [no\_std](#no_std)
  - [Automated tests](#automated-tests)
  - [Maintenance and Release process](#maintenance-and-release-process)

## Introduction

Thanks for your initiative to help improve this crate. We recommend you develop on Linux and use Visual Studio Code with the recommended extensions.

## Quality control

Before you create a pull request make sure
* clippy is happy: `cargo check` and `cargo clippy`.
* all tests pass: `cargo test --all-features` and `cargo test-all-features`.
  * The latter command requires you to have done `cargo install cargo-all-features`. It will test all relevant combinations of features as specified in the `package.metadata.cargo-all-features` section of `Cargo.toml`.
* the code coverage has not deteriorated: `./test_coverage.sh`.
  * Code coverage is currently only supported on Linux and requires the `llvm-tools` which you can install through `rustup component add llvm-tools-preview`.
* there are no spelling errors.
  * You can use the recommended 'Code Spell Checker' VSCode-plugin for that.
* the code is formatted: `cargo fmt`.
  * You can also use 'format on save' in VSCode. Make sure to use a nightly toolchain to be able to use all formatting options specified in `.rustfmt.toml`.

## Visual Studio Code

For most of the aforementioned actions test- and build-tasks have been defined in `tasks.json`.
The recommended VSCode extensions are related to Rust, spell checking, code coverage gutters and markdown. Make sure to install them if you use VSCode.

The code coverage gutters plugin has been pre-configured to look for code coverage information in `coverage/tests.lcov` which is also where the `test_coverage.sh` script writes the coverage information. You may need to press the "ⵔ Watch" button in the status bar to show the gutters after you run the script. There's also an HTML report available at `coverage/html/index.html`.

By default all the features of this crate should be disabled, but this may be inconvenient while developing. Just uncomment the default features line in `Cargo.toml` to enable everything.

## Readers and Writers

This crate is far from complete since there are many readers and writers out there whose usage could benefit from chaining. Feel free to help extend this crate with new features that enable new readers and writers.

## Sources and Sinks

Currently, this crate has several different sources, `FileBuilder`, `ProcessBuilder`, `TcpStreamBuilder` and `VecBuilder`, but only one sink, `BincodeBuilder`. It would be nice if more sources and sinks are added to increase the flexibility when using this crate.

## Buffering

Although this crate supports buffering through `BufReader` and `BufWriter` it doesn't take advantage of the fact that they implement the `BufRead` and `BufWrite` traits respectively. Perhaps this crate can be extended in a way that allows one to take advantage of this extra functionality if the other transformations being chained can also take advantage of it.

## no_std

It would be nice to add a `no_std` feature to enable the use of this crate in embedded programming. See also, [A no_std Rust Environment](https://docs.rust-embedded.org/book/intro/no-std.html).

## Automated tests

There are currently only unit tests, all bundled up in `src/tests.rs`. As the number of tests grows, or integration- and e2e-tests become necessary we need to consider different ways of organizing them.

There are no doc-tests yet, but these would be a welcome addition to provide live documentation. Just make sure to replace existing tests if they cover the same functionality.

## Maintenance and Release process

* Regularly update the crate's dependencies: `cargo update`.
* Make sure the version number is increased, if necessary, in accordance with [semver](https://semver.org/).
* Execute all steps in the [Quality control](#quality-control) section.
* Build and verify the documentation: `cargo doc` -> `target/doc/rw-builder/index.html`.
* Push to the remote `master`.
* `cargo publish`.