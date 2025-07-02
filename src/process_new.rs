use std::fs;

use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::opts_new::OutputFormat;


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

pub fn process_csv_common(input: &str, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // headers.iter() 使用 headers 的迭代器，与 record 的迭代器进行 zip 操作
        // zip 操作会将 headers 和 record 中的元素一一对应起来，形成一个元组集合[(header, value), (header, value), ..., (header, value)...]
        // for i in headers.len() {
        //     let json_value = serde_json::json!({
        //         headers[i]: record[i],
        //     });
        //     ret.push(json_value);
        // }
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();

        // let json_value = headers.iter().zip(record.iter());
        // let json_value = match format {
        //     OutputFormat::Json => iter::collect::serde_json::Value(),
        //     OutputFormat::Yaml => iter.collect::serde_yaml::Value(),
        //     OutputFormat::Toml => iter.collect::toml::Value(),
        // };

        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => toml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}