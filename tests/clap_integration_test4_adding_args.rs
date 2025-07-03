use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliPositionalDerive {
    name: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliPositionalMultDerive {
    name: Vec<String>,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliOptionDerive {
    #[arg(short, long)]
    name: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliOptionMultDerive {
    #[arg(short, long)]
    name: Vec<String>,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliFlags {
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliOptional {
    name: Option<String>,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliDefault {
    #[arg(default_value_t = 2020)]
    port: u16,
}

mod tests {

    use super::*;

    #[test]
    fn test_positional_derive() {
        let args = CliPositionalDerive::try_parse_from(["", "bob"]).unwrap();
        assert_eq!(args.name, "bob");
    }

    #[test]
    fn test_positional_mult_derive() {
        let args = CliPositionalMultDerive::try_parse_from(["", "bob", "john"]).unwrap();
        assert_eq!(args.name[0], "bob");
        assert_eq!(args.name[1], "john");
    }

    #[test]
    fn test_option_derive() {
        let args = CliOptionDerive::try_parse_from(["", "--name", "bob"]).unwrap();
        assert_eq!(args.name, "bob");

        let args = CliOptionDerive::try_parse_from(["", "--name=bob"]).unwrap();
        assert_eq!(args.name, "bob");

        let args = CliOptionDerive::try_parse_from(["", "-n", "bob"]).unwrap();
        assert_eq!(args.name, "bob");

        let args = CliOptionDerive::try_parse_from(["", "-n=bob"]).unwrap();
        assert_eq!(args.name, "bob");

        let args = CliOptionDerive::try_parse_from(["", "-nbob"]).unwrap();
        assert_eq!(args.name, "bob");
    }

    #[test]
    fn test_option_mult_derive() {
        let args = CliOptionMultDerive::try_parse_from(["", "--name", "bob"]).unwrap();
        assert_eq!(args.name[0], "bob");

        let args =
            CliOptionMultDerive::try_parse_from(["", "--name", "bob", "--name", "john"]).unwrap();
        assert_eq!(args.name[0], "bob");
        assert_eq!(args.name[1], "john");

        let args = CliOptionMultDerive::try_parse_from([
            "", "--name", "bob", "--name", "john", "-n", "tom", "-n=chris", "-nsteve",
        ])
        .unwrap();
        assert_eq!(args.name[0], "bob");
        assert_eq!(args.name[1], "john");
        assert_eq!(args.name[2], "tom");
        assert_eq!(args.name[3], "chris");
        assert_eq!(args.name[4], "steve");
    }

    #[test]
    fn test_flag_bool_derive() {
        let args = CliFlags::try_parse_from([""]).unwrap();
        assert_eq!(args.verbose, false);

        let args = CliFlags::try_parse_from(["", "--verbose"]).unwrap();
        assert_eq!(args.verbose, true);
    }

    #[test]
    fn test_optional_derive() {
        let args = CliOptional::try_parse_from([""]).unwrap();
        assert_eq!(args.name.is_none(), true);

        let args = CliOptional::try_parse_from(["", "bob"]).unwrap();
        assert_eq!(args.name.unwrap(), "bob");
    }

    #[test]
    fn test_default_values_derive() {
        let args = CliDefault::try_parse_from([""]).unwrap();
        assert_eq!(args.port, 2020);

        let args = CliDefault::try_parse_from(["", "22"]).unwrap();
        assert_eq!(args.port, 22);
    }
}
