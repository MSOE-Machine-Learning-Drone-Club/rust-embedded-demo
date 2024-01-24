# Demo Project for Compiling Rust to the Esp32-S3

### Currently working on Windows* and Linux

## Installation

### Install all dependencies with [This link](https://dl.espressif.com/dl/esp-idf/) for Windows

### Or, on linux, follow [these instructions](https://github.com/esp-rs/rust-build?tab=readme-ov-file#xtensa-installation)

Once installed, running ```cargo run``` will flash to the connect board and begin monitoring the serial communications. We have compiled this with the std approach so we can (hopefully) use more standard libraries


*: On Windows, you may need to download this file as close to root as possible(directly in the C drive) because there is some weird issue with CMake compiling on windows making large filenames and these files are too long for windows to accept if their in a further out directory

