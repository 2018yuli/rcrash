use std::fs;
use std::fs as stdfs;

fn main() {
    let data = std::fs::read("src/main.rs").unwrap();
    println!("{}", String::from_utf8(data).unwrap());

    let data = fs::read("src/main.rs").unwrap();
    println!("{}", String::from_utf8(data).unwrap());

    let data = stdfs::read("src/main.rs").unwrap();
    println!("{}", String::from_utf8(data).unwrap());
}