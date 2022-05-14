use argh::FromArgs;

#[derive(FromArgs)]
/// Steganographic encoder/decoder
pub struct Flags {
    #[argh(subcommand)]
    pub nested: SubCommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum SubCommand {
    Encode(SubEncode),
    Decode(SubDecode),
}

#[derive(FromArgs)]
/// encode subcommand
#[argh(subcommand, name = "encode")]
pub struct SubEncode {
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

#[derive(FromArgs)]
/// decode subcommand
#[argh(subcommand, name = "decode")]
pub struct SubDecode {
    /// source image filename
    #[argh(positional)]
    pub src: String,

    /// channel/band e.g. R for the RED channel
    #[argh(positional)]
    pub band: Option<String>,
}
