use std::{fs::File, io::Read};

pub fn read_input(input: &str) -> anyhow::Result<Vec<u8>> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
