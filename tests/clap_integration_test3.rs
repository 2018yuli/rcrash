use clap::Parser;

#[derive(Parser)]
#[command(name = "MyApp")]
#[command(version = "1.0")]
#[command(about = "Does awesome things", long_about = None)]
struct Cli {
    #[arg(short, long)]
    two: String,
    #[arg(short, long)]
    one: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli2 {
    #[arg(short, long)]
    two: String,
    #[arg(short, long)]
    one: String,
    #[arg(short, long)]
    zero: bool,
}

mod tests {

    use super::*;

    #[test]
    fn test_config_configuring() {
        let args = Cli::try_parse_from(["", "-o", "0", "-t", "0"]).unwrap();
        assert_eq!(args.one, 0.to_string());
        assert_eq!(args.two, 0.to_string());
        let args2 = Cli2::try_parse_from(["", "-o", "0", "-t", "0"]).unwrap();
        assert_eq!(args2.zero, false);
    }
}