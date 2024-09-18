use std::collections::HashMap;

use crate::{json::token::Number, ParsingError};

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(Number),
    Object(HashMap<String, Value>),
    Null,
    Boolean(bool),
    Array(Vec<Value>),
}

impl TryFrom<Value> for i64 {
    type Error = ParsingError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(Number::Float(number)) => Ok(number as i64),
            _ => Err(ParsingError {
                message: format!("Cannot convert {:?} to f64", value),
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
                message: format!("Cannot convert {:?} to f64", value),
            }),
        }
    }
}

impl TryFrom<Value> for String {
    type Error = ParsingError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(string) => Ok(string),
            _ => Err(ParsingError {
                message: format!("Cannot convert {:?} to String", value),
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
                message: format!("Cannot convert {:?} to boolean", value),
            }),
        }
    }
}

impl TryFrom<Value> for HashMap<String, Value> {
    type Error = ParsingError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(object) => Ok(object),
            _ => Err(ParsingError {
                message: format!("Cannot convert {:?} to Hashmap", value),
            }),
        }
    }
}

impl TryFrom<Value> for Vec<Value> {
    type Error = ParsingError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(array) => Ok(array),
            _ => Err(ParsingError {
                message: format!("Cannot convert {:?} to Vec", value),
            }),
        }
    }
}
