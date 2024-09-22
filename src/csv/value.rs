use crate::ParsingError;

use super::token::{Number, Token};

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(Number),
    Null,
    Boolean(bool),
}

impl From<Token> for Value {
    fn from(value: Token) -> Self {
        match value {
            Token::String(string) => Value::String(string),
            Token::Number(number) => Value::Number(number),
            Token::Null => Value::Null,
            Token::Boolean(boolean) => Value::Boolean(boolean),
        }
    }
}
impl From<&Token> for Value {
    fn from(value: &Token) -> Self {
        match value {
            Token::String(string) => Value::String(string.to_string()),
            Token::Number(number) => Value::Number(number.clone()),
            Token::Null => Value::Null,
            Token::Boolean(boolean) => Value::Boolean(*boolean),
        }
    }
}

impl TryFrom<Value> for String {
    type Error = ParsingError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(string) => Ok(string),
            _ => Err(ParsingError {
                message: format!("value {:?} is not a string", value),
            }),
        }
    }
}

impl TryFrom<Value> for i64 {
    type Error = ParsingError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(Number::Int(number)) => Ok(number),
            _ => Err(ParsingError {
                message: format!("value {:?} is not an integer", value),
            }),
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = ParsingError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(Number::Float(number)) => Ok(number),
            _ => Err(ParsingError {
                message: format!("value {:?} is not a float", value),
            }),
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = ParsingError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(boolean) => Ok(boolean),
            _ => Err(ParsingError {
                message: format!("value {:?} is not a boolean", value),
            }),
        }
    }
}
