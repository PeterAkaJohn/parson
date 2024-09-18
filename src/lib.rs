mod json;
use json::JsonParser;

#[derive(Debug, Clone)]
pub struct ParsingError {
    pub message: String,
}

pub type ParsonResult<T> = Result<T, ParsingError>;

pub struct Parson {
    json: JsonParser,
}

impl Parson {}
