# WebP

A rust wrapper around `libwebp`, version 1.1.0. Do not use. Very very
incomplete. Only supports decoding into RGBA.

## Developing

Standard `cargo` workflows are used:

- `cargo check`
- `cargo build`
- `cargo test`

### First time setup

After cloning, pull in the `libwebp` git submodule:

- `git submodule init`
- `git submodule update`

### Regenerating bindings

Use rust-bindgen (`cargo install bindgen`) to generate the bindings for a
platform:

- `bindgen wrapper.h -o src/ffi_<platform>.rs`
