# Libtransistor-rs

`libtransistor-rs` allows you to write Homebrew apps for the Nintendo Switch in
Rust !

# Usage

First, you'll need to get hold of the switch-friendly rust fork.

```
git clone -b horizon https://github.com/roblabla/rust
```

Once you have this, clone libtransistor-rs. It will have a folder with a simple
project you can use as a base to develop your homebrew.

```
git clone https://github.com/roblabla/libtransistor-rs
```

`simpletest` contains a simple hello world, while `libtransistor-sys` contains
bindings to the `libtransistor` library, a global allocator so you may use Vecs
and Hashmap, and links against `newlib` to satisfy the `libc` dependency.

To build it, you'll need `xargo`, which you can install with
`cargo install xargo`. Then it's as simple as

```
XARGO_RUST_SRC=path/to/rust/src xargo build --target=aarch64-nintendo-horizon-newlib
```

Your binary will arrive, with all its dependencies statically linked, in
`target/aarch64-nintendo-horizon-newlib/debug/simpletest`. You'll need to run
`elf2nxo.py` on it to get an `NRO` that you can load in the Mephisto emulator or
on an actual nintendo switch.

# TODO

- Make idiomatic bindings to the libtransistor-sys crate.
- Port libstd to the Switch
- Find a way to globally install custom target definitions ?
