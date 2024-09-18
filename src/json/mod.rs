mod token;
mod value;

use std::{collections::HashMap, iter::Peekable};

use token::{Token, Tokenizer};
use value::Value;

use crate::{ParsingError, ParsonResult};

pub struct JsonParser {
    tokens: Vec<Token>,
}

impl JsonParser {
    pub fn new(json_string: &str) -> ParsonResult<Self> {
        Ok(Self {
            tokens: Tokenizer::new(&mut json_string.chars())?.tokens,
        })
    }
    pub fn parse(&self) -> ParsonResult<Value> {
        let mut tokens = self.tokens.clone().into_iter().peekable();
        parse_tokens(&mut tokens)
    }
}

fn parse_objects(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> HashMap<String, Value> {
    let mut is_key = true;

    let mut current_key: Option<String> = None;

    let mut value = HashMap::<String, Value>::new();

    while let Some(token) = tokens.next() {
        match token {
            Token::OpenCurlyBracket => {
                if let Some(ref key) = current_key {
                    value.insert(key.to_string(), Value::Object(parse_objects(tokens)));
                }
            }
            Token::CloseCurlyBracket => break,
            Token::OpenSquareBracket => {
                if let Some(ref key) = current_key {
                    value.insert(key.to_string(), Value::Array(parse_arrays(tokens)));
                }
            }
            Token::StringValue(string) => {
                if is_key {
                    current_key = Some(string);
                } else if let Some(ref key) = current_key {
                    value.insert(key.to_string(), Value::String(string));
                }
            }
            Token::NumberValue(number) => {
                if let Some(ref key) = current_key {
                    value.insert(key.to_string(), Value::Number(number));
                }
            }
            Token::Boolean(boolean) => {
                if let Some(ref key) = current_key {
                    value.insert(key.to_string(), Value::Boolean(boolean));
                }
            }
            Token::Null => {
                if let Some(ref key) = current_key {
                    value.insert(key.to_string(), Value::Null);
                }
            }
            Token::Comma => is_key = true,
            Token::Colon => is_key = false,
            _ => {}
        }
    }

    value
}
fn parse_arrays(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Vec<Value> {
    let mut internal_value = Vec::<Value>::new();

    while let Some(token) = tokens.next() {
        match token {
            Token::OpenCurlyBracket => {
                internal_value.push(Value::Object(parse_objects(tokens)));
            }
            Token::OpenSquareBracket => internal_value.push(Value::Array(parse_arrays(tokens))),
            Token::CloseSquareBracket => break,
            Token::StringValue(string) => internal_value.push(Value::String(string)),
            Token::NumberValue(number) => internal_value.push(Value::Number(number)),
            Token::Boolean(boolean) => internal_value.push(Value::Boolean(boolean)),
            Token::Null => internal_value.push(Value::Null),
            _ => {}
        }
    }
    internal_value
}

fn parse_tokens(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParsonResult<Value> {
    let mut value = Value::Null;
    while let Some(token) = tokens.next() {
        match token {
            Token::OpenCurlyBracket => {
                // parse objects
                value = Value::Object(parse_objects(tokens));
            }
            Token::OpenSquareBracket => {
                // parse arrays
                value = Value::Array(parse_arrays(tokens));
            }
            Token::StringValue(string) => value = Value::String(string),
            Token::NumberValue(number) => value = Value::Number(number),
            Token::Boolean(boolean) => {
                value = Value::Boolean(boolean);
            }
            Token::Null => value = Value::Null,
            _ => {
                return Err(ParsingError {
                    message: "Failed to parse object. Is it a valid one?".to_string(),
                })
            }
        };
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_correctly() {
        let json_string = r#"{"number": 1, "string": "val", "boolean": true, "null_value": null, "another_json":{"number": 1, "string": "val", "boolean": true, "null_value": null}}"#;
        let parser = JsonParser::new(json_string).unwrap();
        let value = parser.parse();
        assert!(value.is_ok());
    }

    #[test]
    fn test_parse_failure() {
        let json_string = r#"}"number": 1, "string": "val", "boolean": true, "null_value": null, "another_json":{"number": 1, "string": "val", "boolean": true, "null_value": null}}"#;
        let parser = JsonParser::new(json_string).unwrap();
        let value = parser.parse();
        assert!(value.is_err());

        let json_string = r#"23:4"#;
        let parser = JsonParser::new(json_string);
        assert!(parser.is_err());
    }
}
