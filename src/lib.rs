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
}
