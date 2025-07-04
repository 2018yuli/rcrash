use super::enums::TextSignFormat;
use super::verify_file;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(name = "sig", about = "Sign a text file with a private key")]
    Sig(TextSigOpts),
    #[command(name = "verify", about = "Verify a text file signature")]
    Verify(TextVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSigOpts {
    #[arg(long, value_parser=verify_file, help = "The path to the text file")]
    pub input: String,
    #[arg(long, value_parser=verify_file, help = "The path to the private key")]
    pub key: String,
    #[arg(
        long,
        default_value = "blake3",
        value_parser = parse_formart,
        help = "The format of the key (e.g., Blake3)"
    )]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(help = "The path to the signed text file")]
    signed_file: String,
}

fn parse_formart(format: &str) -> anyhow::Result<TextSignFormat> {
    format.parse()
}
