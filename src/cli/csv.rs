use super::enums::OutputFormat;
use super::verify_input_file;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser=verify_input_file)]
    pub input: String,

    // #[arg(short, long, default_value="assets/output.json")] // "output.json".into()
    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, value_parser = parse_formart, default_value="json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value = ",")]
    pub delimiter: String,

    #[arg(short = 'a', long, default_value_t = true)]
    pub header: bool,
}

fn parse_formart(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}
