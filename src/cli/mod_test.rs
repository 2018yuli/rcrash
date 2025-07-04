#[cfg(test)]
mod tests {
    use crate::cli::verify_file;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("file does not exist"));
        assert_eq!(verify_file("src/cli/mod.rs"), Ok("src/cli/mod.rs".into()));
        assert_eq!(verify_file("not_exist"), Err("file does not exist"));
    }
}
