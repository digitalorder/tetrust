Tetris engine on Rust
=====================

Prerequisites
-------------

This project is written on Rust. Follow [official instructions](https://www.rust-lang.org/tools/install) to install Rust environment on your PC.

Deployment
----------

Just clone:

```
$ git clone https://github.com/digitalorder/tetrust.git
$ cd tetrust
```

Build & Run
-----------

Use cargo to build the source:

```
$ cargo build --release
```

And run:

```
$ cargo run
```

Alternatevily:

```
$ target/release/tetrust
```

Tests
-----

All tests are expected to be green:

```
$ cargo test
```
