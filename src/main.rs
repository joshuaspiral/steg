use color_eyre::Result;
use std::env;
use steg::{cli::*, *};

fn main() -> Result<()> {
    if env::var("RUST_LIB_BACKTRACE").is_err() {
        env::set_var("RUST_LIB_BACKTRACE", "1");
    }
    color_eyre::install()?;

    let flags: Flags = argh::from_env();

    match flags.nested {
        SubCommand::Encode(SubEncode { src, target, band }) => {
            let band = band.unwrap_or_else(|| String::from("r"));
            encode(&src, &target, &band)
        }
        SubCommand::Decode(SubDecode { src, band }) => {
            let band = band.unwrap_or_else(|| String::from("r"));
            decode(&src, &band)
        }
    }
}
