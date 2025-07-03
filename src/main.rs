// rcrash csv -i input.csv -o output .json --header -d ','
// cargo run -- csv -i test.csv
// cargo run csv -i assets/juventus.csv
// cargo run csv -i assets/juventus.csv --format yaml
// cargo run csv -i assets/juventus.csv --format toml
// cargo run gen-password --length 10 --uppercase --lowercase --numbers --symbols

use clap::Parser;

mod opts;

// using mod.rs, with is not necessary since rust 2018
// mod process;
// use crate::{opts::{Opts, SubCommand}, process::process_csv};

// using lib.rs
use rcrash::{process_csv_common, process_generate_password, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("assets/output.{}", opts.format)
            };
            process_csv_common(&opts.input, &output, opts.format)?
        }
        SubCommand::GenPassword(opts) => process_generate_password(
            opts.length,
            opts.uppercase,
            opts.lowercase,
            opts.numbers,
            opts.symbols,
        )?,
    }
    Ok(())
}
