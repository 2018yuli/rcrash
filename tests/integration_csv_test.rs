use csv::ReaderBuilder;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u32,
}

mod tests {
    use super::*;
    use std::io::Cursor;
    use std::path::Path;

    #[test]
    fn test_csv_from_file_processing() {
        // Define the path to the CSV file.
        let path = Path::new("assets/juventus.csv");

        // Open the CSV file.
        let file = File::open(path).unwrap();

        // Initialize the CSV reader.
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

        // Iterate over each record in the CSV.
        let mut records = rdr.deserialize::<Player>();

        // First record (Wojciech Szczesny)
        let record: Player = records.next().unwrap().unwrap();
        assert_eq!(record.name, "Wojciech Szczesny");
        assert_eq!(record.position, "Goalkeeper");
        assert_eq!(record.dob, "Apr 18, 1990 (29)");
        assert_eq!(record.nationality, "Poland");
        assert_eq!(record.kit_number, 1);

        // Second record (Mattia Perin)
        let record: Player = records.next().unwrap().unwrap();
        assert_eq!(record.name, "Mattia Perin");
        assert_eq!(record.position, "Goalkeeper");
        assert_eq!(record.dob, "Nov 10, 1992 (26)");
        assert_eq!(record.nationality, "Italy");
        assert_eq!(record.kit_number, 37);
    }

    #[test]
    fn test_csv_processing() {
        let data = b"name,age\nAlice,30\nBob,25\n";
        let mut rdr = csv::Reader::from_reader(Cursor::new(data));

        let records: Vec<_> = rdr.records().collect::<Result<_, _>>().unwrap();

        assert_eq!(records[0].get(0), Some("Alice"));
        assert_eq!(records[0].get(1), Some("30"));

        assert_eq!(records[1].get(0), Some("Bob"));
        assert_eq!(records[1].get(1), Some("25"));
    }

    #[test]
    fn test_empty_csv() {
        let data = b"";
        let mut rdr = csv::Reader::from_reader(Cursor::new(data));

        let records: Vec<_> = rdr.records().collect::<Result<_, _>>().unwrap();

        assert!(records.is_empty(), "Expected no records");
    }

    #[test]
    fn test_csv_with_custom_delimiter() {
        let data = b"name|age\nAlice|30\nBob|25\n";

        let mut rdr = ReaderBuilder::new()
            .has_headers(true) // Indicates that the first row contains the header
            .delimiter(b'|') // Custom delimiter: '|'
            .from_reader(Cursor::new(data));

        // Get headers
        let headers = rdr.headers().unwrap();
        assert_eq!(headers.get(0), Some("name"));
        assert_eq!(headers.get(1), Some("age"));

        // Read the records (data rows)
        let records: Vec<_> = rdr.records().collect::<Result<_, _>>().unwrap();

        assert_eq!(records[0].get(0), Some("Alice"));
        assert_eq!(records[0].get(1), Some("30"));

        assert_eq!(records[1].get(0), Some("Bob"));
        assert_eq!(records[1].get(1), Some("25"));
    }
}
