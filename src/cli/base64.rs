use clap::Parser;

use super::enums::Base64Format;
use super::verify_input_file;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct EncodeOpts {
    #[arg(long, value_parser=verify_input_file, default_value="-",  help = "The string to encode")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_formart, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct DecodeOpts {
    #[arg(long, value_parser=verify_input_file, default_value="-", help = "The base64 string to decode")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_formart, default_value = "uri")]
    pub format: Base64Format,
}

fn parse_base64_formart(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}
