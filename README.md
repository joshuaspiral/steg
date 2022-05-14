# Steg

Steganographic encoder/decoder with one bit per byte on one channel

## Usage

Building `steg` from source requires a functional [Rust](//rust-lang.org) toolchain. You can install Rust at with [rustup](//rustup.rs).

To build from source:

```sh
cargo build --release
```

## Options

```
Usage: steg <src> <target> [<band>] [-d]

Steganographic encoder/decoder

Positional Arguments:
  src               source image filename
  target            target filename
  band              channel/band e.g. R for the RED channel

Options:
  -d, --decode      to switch mode to decode
  --help            display usage information
```

### Licensing

`steg` is licensed under the [MIT License](//opensource.org/licenses/MIT).
