
// rcrash csv -i input.csv -o output .json --header -d ','
// cargo run -- csv -i test.csv
// cargo run csv -i assets/juventus.csv

use std::path::Path;

use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};

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
    #[command(name="csv", about="Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser=verify_input_file)]
    input: String,

    #[arg(short, long, default_value="output.json")] // "output.json".into()
    output: String,

    #[arg(short, long, default_value=",")]
    delimiter: String,

    #[arg(short = 'a', long, default_value_t = true)]
    header: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u32,
}

fn verify_input_file(filename : &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("file does not exist")
    }
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            // 加入 anyhow 之后，? 任何的 Err 都可以转换为 anyhow::Error
            let mut reader = Reader::from_path(opts.input)?;
            // let players = reader
            //     .deserialize()
            //     .map(|record| record.unwrap())
            //     .collect::<Vec<Player>>();
            for result in reader.deserialize() {
                let player: Player = result?;
                println!("{:?}", player);
            }
        },
    }
    Ok(())
}
