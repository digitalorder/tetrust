Tetris engine on Rust
=====================

![build-macos](https://github.com/digitalorder/tetrust/workflows/Cargo%20Build%20&%20Test%20on%20MacOs/badge.svg?branch=master&event=push) ![build-ubuntu](https://github.com/digitalorder/tetrust/workflows/Cargo%20Build%20&%20Test%20on%20Ubuntu/badge.svg?branch=master&event=push)

Prerequisites
-------------

This project is written on Rust. Follow [official instructions](https://www.rust-lang.org/tools/install) to install Rust environment on your PC.

This project WILL NOT work on Windows, because I'm too lazy to port view formatter there.

Deployment
----------

Just clone:

```shell
$ git clone https://github.com/digitalorder/tetrust.git
Cloning into 'tetrust'...
$ cd tetrust
```

Build & Run
-----------

Use cargo to build the source:

```shell
$ cargo build --release
```

And run:

```shell
$ cargo run
```

Alternatevily:

```shell
$ target/release/tetrust
```

Tests
-----

All tests are expected to be green:

```shell
$ cargo test
```
