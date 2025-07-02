use std::{fmt, path::Path, str::FromStr};

use clap::Parser;


/*
描述命令行中的 根命令 的参数
*/
#[derive(Debug, Parser)]
#[command(name="cli", version, author, about, long_about = None)]
pub struct OptsNew {
    #[command(subcommand)]
    pub cmd: SubCommandNew,
}

/*
SubCommand 是一个枚举类型，表示所有的子命令。这个例子中只有一个子命令 csv
*/
#[derive(Debug, Parser)]
pub enum SubCommandNew {
    #[command(name="csv", about="Show CSV, or convert CSV to other formats")]
    Csv(CsvOptsNew),
}

#[derive(Debug, Parser)]
pub struct CsvOptsNew {
    #[arg(short, long, value_parser=verify_input_file)]
    pub input: String,

    // #[arg(short, long, default_value="assets/output.json")] // "output.json".into()
    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, value_parser = parse_formart, default_value="json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value=",")]
    pub delimiter: String,

    #[arg(short = 'a', long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

fn verify_input_file(filename : &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("file does not exist")
    }
}

fn parse_formart(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            // "toml" => Ok(OutputFormat::Toml),
            v => anyhow::bail!("unsupported format: {}", v),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}