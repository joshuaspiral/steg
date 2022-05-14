use argh::FromArgs;

#[derive(FromArgs)]
/// Steganographic encoder/decoder
pub struct Flags {
    /// to switch mode to decode
    #[argh(switch, short = 'd')]
    pub decode: bool,

    /// source image filename
    #[argh(positional)]
    pub src: String,

    /// target filename
    #[argh(positional)]
    pub target: String,

    /// channel/band e.g. R for the RED channel
    #[argh(positional)]
    pub band: Option<String>,
}

pub fn validate_args(flags: &Flags) {
    if let Some(band) = &flags.band {
        if !matches!(band.as_str(), "r" | "g" | "b") {
            eprintln!("band should be r, g or b");
            std::process::exit(1);
        }
    }
}
