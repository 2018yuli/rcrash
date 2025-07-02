use std::{fmt, path::Path, str::FromStr};

use clap::Parser;


/*
描述命令行中的 根命令 的参数
*/
#[derive(Debug, Parser)]
#[command(name="cli", version, author, about, long_about = None)]
pub struct OptsRandom {
    #[command(subcommand)]
    pub cmd: SubCommandRandom,
}

/*
SubCommand 是一个枚举类型，表示所有的子命令。这个例子中只有一个子命令 csv
*/
#[derive(Debug, Parser)]
pub enum SubCommandRandom {
    #[command(name="csv", about="Show CSV, or convert CSV to other formats")]
    Csv(CsvOptsRandom),

    #[command(name="gen-password", about="Generate a password")]
    GenPassword(GenPasswordOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOptsRandom {
    #[arg(short, long, value_parser=verify_input_file)]
    pub input: String,

    // #[arg(short, long, default_value="assets/output.json")] // "output.json".into()
    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, value_parser = parse_formart, default_value="json")]
    pub format: OutputFormatRandom,

    #[arg(short, long, default_value=",")]
    pub delimiter: String,

    #[arg(short = 'a', long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Parser)]
pub struct GenPasswordOpts {
    #[arg(short, long, default_value="12")]
    pub length: u8,

    #[arg(long, default_value_t=true)]
    pub uppercase: bool,

    #[arg(long, default_value_t=true)]
    pub lowercase: bool,

    #[arg(long, default_value_t=true)]
    pub numbers: bool,

    #[arg(long, default_value_t=true)]
    pub symbols: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormatRandom {
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

fn parse_formart(format: &str) -> Result<OutputFormatRandom, anyhow::Error> {
    format.parse()
}

impl From<OutputFormatRandom> for &'static str {
    fn from(format: OutputFormatRandom) -> Self {
        match format {
            OutputFormatRandom::Json => "json",
            OutputFormatRandom::Yaml => "yaml",
            OutputFormatRandom::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormatRandom {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "json" => Ok(OutputFormatRandom::Json),
            "yaml" => Ok(OutputFormatRandom::Yaml),
            // "toml" => Ok(OutputFormatRandom::Toml),
            v => anyhow::bail!("unsupported format: {}", v),
        }
    }
}

impl fmt::Display for OutputFormatRandom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}