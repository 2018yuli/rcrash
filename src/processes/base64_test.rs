#[cfg(test)]
mod tests {

    use crate::{process_decode, process_encode};

    #[test]
    fn process_encode_test() {
        let result = process_encode("Cargo.toml", crate::cli::Base64Format::Standard);

        assert!(result.is_ok(), "Expected Ok but got {:?}", result);
    }

    #[test]
    fn process_encode_uri_test() {
        let result = process_encode("Cargo.toml", crate::cli::Base64Format::Uri);

        assert!(result.is_ok(), "Expected Ok but got {:?}", result);
    }

    #[test]
    fn process_decode_test() {
        let result = process_decode(
            "tests/fuxtures/test.txt",
            crate::cli::Base64Format::Standard,
        );

        assert!(result.is_ok(), "Expected Ok but got {:?}", result);
    }

    #[test]
    fn process_decode_uri_test() {
        let result = process_decode("tests/fuxtures/test.txt", crate::cli::Base64Format::Uri);

        assert!(result.is_ok(), "Expected Ok but got {:?}", result);
    }
}
