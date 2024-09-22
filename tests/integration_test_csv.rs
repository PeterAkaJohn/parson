use std::fs::read;

use parson::{ParsingError, Parson, ParsonResult};

#[test]
fn read_csv_data_from_test_file() -> ParsonResult<()> {
    let mut crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    crate_dir.push_str("/tests/test_files/MOCK_DATA.csv");
    let bytes = read(crate_dir).map_err(|_| ParsingError {
        message: "failed to read test file".to_string(),
    })?;

    let parsed_csv = Parson::parse_csv_with_bytes(&bytes);

    assert!(parsed_csv.is_ok());

    Ok(())
}
