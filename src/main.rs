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
        SubCommand::Encode(SubEncode { src, target }) => Image::new(&src)?.encode(&target),
        SubCommand::Decode(SubDecode { src }) => Ok(Image::new(&src)?.decode()),
        SubCommand::Wipe(SubWipe { target }) => {
            let mut img = Image::new(&target)?;
            img.wipe();
            img.save(&target)
        }
    }
}
