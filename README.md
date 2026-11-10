# wiiuse-rs

A Rust API around the native `wiiuse` library.

## Requirements

- Rust and Cargo
- CMake
- A C compiler
- Clang/libclang for bindgen
- The `vendor/wiiuse` source tree used by this repository

## Build

```bash
cargo build
```

## Examples

```bash
cargo run --example single_remote
```

The example prints button press transitions and streams accelerometer readings
continuously at 20 Hz.

## License

The native `wiiuse` library is GPL-3.0-or-later. Check the licensing terms
before redistributing binaries or derivative works.
