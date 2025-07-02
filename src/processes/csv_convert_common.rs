use std::fs;

use csv::Reader;
use serde_json::Value;

use crate::opts_random::OutputFormatRandom;


pub fn process_csv_common(input: &str, output: &str, format: OutputFormatRandom) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormatRandom::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormatRandom::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormatRandom::Toml => toml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}