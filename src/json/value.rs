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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{json::token::Number, ParsonResult};

    use super::Value;

    #[test]
    fn test_conversions() {
        let value = bool::try_from(Value::Boolean(false));
        assert!(value.is_ok());
        assert!(!value.unwrap());

        let value = f64::try_from(Value::Number(Number::Float(32.2)));
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), 32.2);

        let value = i64::try_from(Value::Number(Number::Float(32.2)));
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), 32);

        let value: ParsonResult<String> = Value::String("amazing".to_string()).try_into();
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), "amazing".to_string());

        let value: ParsonResult<Vec<Value>> =
            Value::Array(vec![Value::Boolean(false), Value::Boolean(true)]).try_into();
        assert!(value.is_ok());
        assert_eq!(value.unwrap().len(), 2);

        let value: ParsonResult<HashMap<String, Value>> =
            Value::Object(HashMap::from([("test".to_string(), Value::Boolean(true))])).try_into();

        assert!(value.is_ok());
        let unwrapped_value = value.unwrap();
        assert_eq!(unwrapped_value.keys().len(), 1);
        assert!(unwrapped_value
            .get("test")
            .is_some_and(|k| matches!(k, Value::Boolean(true))));
    }
}
