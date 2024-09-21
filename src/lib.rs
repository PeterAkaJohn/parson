mod csv;
use std::collections::HashMap;

use csv::{CsvParser, Value as CsvValue};
mod json;
use json::{JsonParser, Value};

#[derive(Debug, Clone)]
pub struct ParsingError {
    pub message: String,
}

pub type ParsonResult<T> = Result<T, ParsingError>;

#[derive(Debug)]
pub struct Parson {}

impl Parson {
    pub fn parse_json(json_string: &str) -> ParsonResult<Value> {
        let json_parser = JsonParser::new(json_string)?;
        json_parser.parse()
    }

    pub fn parse_csv(csv_string: &str) -> ParsonResult<Vec<HashMap<String, CsvValue>>> {
        let csv_parser = CsvParser::new(csv_string.as_bytes())?;
        csv_parser.parse()
    }
}
