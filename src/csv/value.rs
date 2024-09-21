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
