# wiimote-rust

Rust wrapper around the native wiiuse library.

## Requirements

- Rust
- CMake
- C compiler
- Git

### macOS

brew install cmake llvm

export LIBCLANG_PATH="$(brew --prefix llvm)/lib"

### Linux

Install CMake, Clang, BlueZ development headers
and a C compiler.

For Ubuntu:

sudo apt install cmake clang libclang-dev \
libbluetooth-dev build-essential

### Windows

Install:

- Visual Studio C++ Build Tools
- CMake
- LLVM/Clang

## Build

cargo build

## Run

cargo run

## Example

cargo run --example accelerometer

## License

wiiuse is GPL-3.0-or-later.
