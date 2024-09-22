use std::fs::read;

use parson::{ParsingError, Parson, ParsonResult};

#[test]
fn read_json_data_from_test_file() -> ParsonResult<()> {
    let mut crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    crate_dir.push_str("/tests/test_files/MOCK_DATA.json");
    let bytes = read(crate_dir).map_err(|_| ParsingError {
        message: "failed to read test file".to_string(),
    })?;

    let parsed_json = Parson::parse_json_with_bytes(&bytes);

    assert!(parsed_json.is_ok());

    Ok(())
}
