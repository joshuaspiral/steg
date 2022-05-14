use color_eyre::Result;
use colored::Colorize;
use std::{env, process::exit};
use steg::{cli::*, *};

fn main() -> Result<()> {
    if env::var("RUST_LIB_BACKTRACE").is_err() {
        env::set_var("RUST_LIB_BACKTRACE", "1");
    }
    color_eyre::install()?;

    let flags: Flags = argh::from_env();

    match flags.nested {
        SubCommand::Encode(SubEncode { src, target, band }) => {
            encode(&src, &target, &validate_band(band))
        }
        SubCommand::Decode(SubDecode { src, band }) => decode(&src, &validate_band(band)),
        SubCommand::Wipe(SubWipe { target, band }) => wipe(&target, &validate_band(band)),
    }
}

fn validate_band(band: Option<String>) -> String {
    let band = band.unwrap_or_else(|| String::from("r"));
    if matches!(band.as_str(), "r" | "g" | "b") {
        band
    } else {
        eprintln!("{}", format!("Invalid band: {band}").red().bold());
        exit(1);
    }
}
