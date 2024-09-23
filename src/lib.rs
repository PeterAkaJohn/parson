mod csv;
use std::collections::HashMap;

use csv::{CsvParser, Value as CsvValue};
mod json;
use json::{JsonParser, Value as JsonValue};

#[derive(Debug, Clone)]
pub struct ParsingError {
    pub message: String,
}

pub type ParsonResult<T> = Result<T, ParsingError>;

#[derive(Debug)]
pub struct Parson {}

impl Parson {
    pub fn parse_json(json_string: &str) -> ParsonResult<JsonValue> {
        let json_parser = JsonParser::new(json_string.as_bytes())?;
        json_parser.parse()
    }

    pub fn parse_json_with_bytes(bytes: &[u8]) -> ParsonResult<JsonValue> {
        let json_parser = JsonParser::new(bytes)?;
        json_parser.parse()
    }

    pub fn parse_csv(csv_string: &str) -> ParsonResult<Vec<HashMap<String, CsvValue>>> {
        let csv_parser = CsvParser::new(csv_string.as_bytes())?;
        csv_parser.parse()
    }

    pub fn parse_csv_with_bytes(bytes: &[u8]) -> ParsonResult<Vec<HashMap<String, CsvValue>>> {
        let csv_parser = CsvParser::new(bytes)?;
        csv_parser.parse()
    }
}
