// cargo nextest run
// rcrash csv -i input.csv -o output .json --header -d ','
// cargo run -- csv -i test.csv
// cargo run csv -i assets/juventus.csv
// cargo run csv -i assets/juventus.csv --format yaml
// cargo run csv -i assets/juventus.csv --format toml
// cargo run gen-password --length 10 --uppercase --lowercase --numbers --symbols
// cargo run base64 encode (回车  ctrl+D)
// cargo run base64 encode --input Cargo.toml
// cargo run base64 decode
//
// cargo run gen-password --length 32 > ./assets/key
// cargo run text sig --key ./assets/key
// cargo run text verify --key ./assets/key --input ./assets/sign.txt --sig RZJxiZleHv708HuxuQN68D1vVNyqnZGmVVYvFmU8SDc

use clap::Parser;
use zxcvbn::zxcvbn;

// using mod.rs, with is not necessary since rust 2018
// mod process;
// use crate::{opts::{Opts, SubCommand}, process::process_csv};

// using lib.rs,
use rcrash::{
    process_csv_common, process_decode, process_encode, process_generate_key,
    process_generate_password, process_sign, process_verify, Base64SubCommand, Opts, SubCommand,
    TextSubCommand,
};

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
        SubCommand::GenPassword(opts) => {
            let password = process_generate_password(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbols,
            )?;
            // 打印密码强度
            let estimate = zxcvbn(&password, &[]);
            // 打印到 stderr 中，便于 pipeline 纯净输出
            eprintln!("strength: {}", estimate.score());
            println!("{password}")
        }
        SubCommand::Base64(opts) => match opts {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{encoded}")
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                let decoded = String::from_utf8_lossy(&decoded);
                println!("{decoded}")
            }
        },
        SubCommand::Text(opts) => match opts {
            TextSubCommand::Sig(opts) => process_sign(&opts.input, &opts.key, opts.format)?,
            TextSubCommand::Verify(opts) => {
                let success = process_verify(&opts.input, &opts.key, &opts.sig, opts.format)?;
                println!("Verification result: {success}")
            }
            TextSubCommand::GenKey(opts) => process_generate_key(&opts.output, opts.format)?,
        },
    }
    Ok(())
}
