# Steg

Steganographic encoder/decoder with one bit per byte on one channel

## Usage

Building `steg` from source requires a functional [Rust](//rust-lang.org) toolchain. You can install Rust with [rustup](//rustup.rs).

To build from source:

```sh
cargo build --release
```

To install to path:
```sh
cargo install --path .
```

## Options

```
Usage: steg <command> [<args>]

Steganographic encoder/decoder

Options:
  --help            display usage information

Commands:
  encode            encode subcommand
  decode            decode subcommand
  wipe              wipe subcommand
```

### Licensing

`steg` is licensed under the [MIT License](//opensource.org/licenses/MIT).
