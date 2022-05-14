use std::error::Error;
use steg::cli::*;

fn main() -> Result<(), Box<dyn Error>> {
    let flags: Flags = argh::from_env();
    validate_args(&flags);
    if flags.decode {
        steg::decode(&flags)
    } else {
        steg::encode(&flags)
    }
}
