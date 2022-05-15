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
    Wipe(SubWipe),
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
}

#[derive(FromArgs)]
/// decode subcommand
#[argh(subcommand, name = "decode")]
pub struct SubDecode {
    /// source image filename
    #[argh(positional)]
    pub src: String,
}

#[derive(FromArgs)]
/// wipe subcommand
#[argh(subcommand, name = "wipe")]
pub struct SubWipe {
    /// file to wipe
    #[argh(positional)]
    pub target: String,
}
