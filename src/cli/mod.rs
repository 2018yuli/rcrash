use std::path::Path;

use clap::Parser;

mod base64;
mod csv;
mod enums;
mod genpass;

// tests
mod mod_test;

pub use self::base64::Base64SubCommand;
use self::csv::CsvOpts;
use self::genpass::GenPasswordOpts;

pub use enums::*;

/*
描述命令行中的 根命令 的参数
*/
#[derive(Debug, Parser)]
#[command(name="cli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

/*
SubCommand 是一个枚举类型，表示所有的子命令。这个例子中只有一个子命令 csv
*/
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),

    #[command(name = "gen-password", about = "Generate a password")]
    GenPassword(GenPasswordOpts),

    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("file does not exist")
    }
}
