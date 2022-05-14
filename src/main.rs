use color_eyre::Result;
use std::env;
use steg::cli::*;

fn main() -> Result<()> {
    if env::var("RUST_LIB_BACKTRACE").is_err() {
        env::set_var("RUST_LIB_BACKTRACE", "1");
    }
    color_eyre::install()?;

    let flags: Flags = argh::from_env();
    validate_args(&flags);

    if flags.decode {
        steg::decode(&flags)
    } else {
        steg::encode(&flags)
    }
}
