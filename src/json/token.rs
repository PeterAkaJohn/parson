use std::{iter::Peekable, str::Chars};

use crate::{ParsingError, ParsonResult};

#[derive(Debug, Clone)]
pub enum Number {
    Float(f64),
}

#[derive(Debug, Clone)]
pub enum Token {
    OpenCurlyBracket,
    CloseCurlyBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    StringValue(String),
    NumberValue(Number),
    Comma,
    Colon,
    Whitespace,
    Boolean(bool),
    Null,
}

#[derive(Debug)]
pub struct Tokenizer {
    pub tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(chars: &mut Chars) -> ParsonResult<Self> {
        Ok(Self {
            tokens: Self::tokenize(chars)?,
        })
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    fn parse_string(peekable_chars: &mut Peekable<&mut Chars>) -> ParsonResult<Token> {
        let mut string_value = String::new();
        let mut complete = false;
        let mut previous_value: Option<char> = None;
        for next_value in peekable_chars.by_ref() {
            if next_value != '"' {
                string_value.push(next_value);
                previous_value = Some(next_value);
            } else if previous_value == Some('\\') {
                string_value.push(next_value);
            } else {
                complete = true;
                break;
            }
        }

        if !complete {
            return Err(ParsingError {
                message: "Not a valid json".to_string(),
            });
        }
        Ok(Token::StringValue(string_value))
    }

    fn parse_number(val: char, peekable_chars: &mut Peekable<&mut Chars>) -> ParsonResult<Token> {
        let mut string_value = val.to_string();

        while let Some(next_value) = peekable_chars.peek() {
            if *next_value == ',' {
                break;
            }

            if (*next_value).is_numeric()
                || *next_value == '.'
                || *next_value == 'E'
                || *next_value == 'e'
            {
                string_value.push(*next_value);
            } else {
                return Err(ParsingError {
                    message: format!(
                        "character {} is not allowed when parsing a number",
                        next_value
                    ),
                });
            }

            peekable_chars.next();
        }

        let number: f64 = string_value.parse().map_err(|e| ParsingError {
            message: format!("Parsing Failed {:?}", e),
        })?;

        Ok(Token::NumberValue(Number::Float(number)))
    }

    fn parse_boolean(val: char, peekable_chars: &mut Peekable<&mut Chars>) -> ParsonResult<Token> {
        let expected = if val == 't' { "rue" } else { "alse" };

        for expected_char in expected.chars() {
            if peekable_chars.next() != Some(expected_char) {
                return Err(ParsingError {
                    message: "Failed to parse boolean value".to_string(),
                });
            }
        }

        Ok(Token::Boolean(val == 't'))
    }

    fn parse_null(peekable_chars: &mut Peekable<&mut Chars>) -> ParsonResult<Token> {
        let expected = "ull";
        for expected_char in expected.chars() {
            if peekable_chars.next() != Some(expected_char) {
                return Err(ParsingError {
                    message: "Failed to parse null value".to_string(),
                });
            }
        }

        Ok(Token::Null)
    }

    fn tokenize(chars: &mut Chars) -> ParsonResult<Vec<Token>> {
        let mut tokens = vec![];
        let mut peekable_chars = chars.peekable();
        while let Some(character) = peekable_chars.next() {
            let token = match character {
                '{' => Token::OpenCurlyBracket,
                '}' => Token::CloseCurlyBracket,
                '[' => Token::OpenSquareBracket,
                ']' => Token::CloseSquareBracket,
                ',' => Token::Comma,
                ':' => Token::Colon,
                '"' => Self::parse_string(&mut peekable_chars)?,
                val @ ('0'..='9' | '-' | '+') => Self::parse_number(val, &mut peekable_chars)?,
                val @ ('t' | 'f') => Self::parse_boolean(val, &mut peekable_chars)?,
                'n' => Self::parse_null(&mut peekable_chars)?,
                _ => {
                    // this is mainly whitespace can be ignored
                    Token::Whitespace
                }
            };
            tokens.push(token);
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        json::token::{Number, Token},
        ParsingError,
    };

    use super::Tokenizer;

    #[test]
    fn correctly_parse_brackets_and_string() {
        let string = r#"[]{}"something""somethingelse"  "#;

        let tokenizer = Tokenizer::new(&mut string.chars());

        assert!(tokenizer.is_ok());

        let tokenizer = tokenizer.unwrap();

        let tokens = tokenizer.tokens;

        assert_eq!(tokens.len(), 8);

        assert!(matches!(tokens[0], Token::OpenSquareBracket));
        assert!(matches!(tokens[1], Token::CloseSquareBracket));
        assert!(matches!(tokens[2], Token::OpenCurlyBracket));
        assert!(matches!(tokens[3], Token::CloseCurlyBracket));

        assert!(matches!(tokens[4], Token::StringValue(..)));
        let value = if let Token::StringValue(val) = &tokens[4] {
            val
        } else {
            panic!("String value does not match expected something");
        };
        assert_eq!(value, "something");
        assert!(matches!(tokens[5], Token::StringValue(..)));
        let value = if let Token::StringValue(val) = &tokens[5] {
            val
        } else {
            panic!("String value does not match expected something");
        };
        assert_eq!(value, "somethingelse");

        assert!(matches!(tokens[6], Token::Whitespace));
        assert!(matches!(tokens[7], Token::Whitespace));
    }

    #[test]
    fn return_error_if_string_not_complete() {
        let string = r#"[]{}"something"somethingelse"  "#;

        let tokenizer = Tokenizer::new(&mut string.chars());

        assert!(tokenizer.is_err());
    }
    #[test]
    fn correctly_tokenize_boolean() {
        let string = r#"false,true, false, true"#;

        let tokenizer = Tokenizer::new(&mut string.chars());

        assert!(tokenizer.is_ok());

        let tokenizer = tokenizer.unwrap();

        let tokens = tokenizer.tokens;

        assert_eq!(tokens.len(), 9);

        assert!(matches!(tokens[0], Token::Boolean(false)));
        assert!(matches!(tokens[1], Token::Comma));
        assert!(matches!(tokens[2], Token::Boolean(true)));
        assert!(matches!(tokens[3], Token::Comma));
        assert!(matches!(tokens[4], Token::Whitespace));
        assert!(matches!(tokens[5], Token::Boolean(false)));
        assert!(matches!(tokens[6], Token::Comma));
        assert!(matches!(tokens[7], Token::Whitespace));
        assert!(matches!(tokens[8], Token::Boolean(true)));
    }
    #[test]
    fn unsuccessfully_tokenize_boolean() {
        let string = r#"folse,true, false, true"#;

        let tokenizer = Tokenizer::new(&mut string.chars());

        assert!(tokenizer.is_err());
    }
    #[test]
    fn correctly_tokenize_null() {
        let string = r#"null,null, null"#;

        let tokenizer = Tokenizer::new(&mut string.chars());

        assert!(tokenizer.is_ok());

        let tokenizer = tokenizer.unwrap();

        let tokens = tokenizer.tokens;

        assert_eq!(tokens.len(), 6);

        assert!(matches!(tokens[0], Token::Null));
        assert!(matches!(tokens[1], Token::Comma));
        assert!(matches!(tokens[2], Token::Null));
        assert!(matches!(tokens[3], Token::Comma));
        assert!(matches!(tokens[4], Token::Whitespace));
        assert!(matches!(tokens[5], Token::Null));
    }
    #[test]
    fn unsuccessfully_tokenize_null() {
        let string = r#"noll,null, null"#;

        let tokenizer = Tokenizer::new(&mut string.chars());

        assert!(tokenizer.is_err());
    }
    #[test]
    fn correctly_tokenize_number() {
        let string = r#"1.234234,  23, 4.4,-0.23,+23.43"#;

        let tokenizer = Tokenizer::new(&mut string.chars());

        assert!(tokenizer.is_ok());

        let tokenizer = tokenizer.unwrap();

        let tokens = tokenizer.tokens;

        assert_eq!(tokens.len(), 12);

        assert!(matches!(tokens[0], Token::NumberValue(Number::Float(..))));

        let value = if let Token::NumberValue(Number::Float(val)) = &tokens[0] {
            val
        } else {
            panic!("This value must be a number, we asserted it one line above");
        };

        assert_eq!(*value, 1.234234);
        assert!(matches!(tokens[1], Token::Comma));
        assert!(matches!(tokens[2], Token::Whitespace));
        assert!(matches!(tokens[3], Token::Whitespace));
        assert!(matches!(tokens[4], Token::NumberValue(Number::Float(..))));

        let value = if let Token::NumberValue(Number::Float(val)) = &tokens[4] {
            val
        } else {
            panic!("This value must be a number, we asserted it one line above");
        };

        assert_eq!(*value, 23.0);
        assert!(matches!(tokens[5], Token::Comma));
        assert!(matches!(tokens[6], Token::Whitespace));

        assert!(matches!(tokens[7], Token::NumberValue(Number::Float(..))));

        let value = if let Token::NumberValue(Number::Float(val)) = &tokens[7] {
            val
        } else {
            panic!("This value must be a number, we asserted it one line above");
        };
        assert_eq!(*value, 4.4);
        assert!(matches!(tokens[8], Token::Comma));

        assert!(matches!(tokens[9], Token::NumberValue(Number::Float(..))));

        let value = if let Token::NumberValue(Number::Float(val)) = &tokens[9] {
            val
        } else {
            panic!("This value must be a number, we asserted it one line above");
        };
        assert_eq!(*value, -0.23);

        assert!(matches!(tokens[10], Token::Comma));
        assert!(matches!(tokens[11], Token::NumberValue(Number::Float(..))));

        let value = if let Token::NumberValue(Number::Float(val)) = &tokens[11] {
            val
        } else {
            panic!("This value must be a number, we asserted it one line above");
        };
        assert_eq!(*value, 23.43);
    }

    #[test]
    fn unsuccessfully_tokenize_number() {
        let string = r#"34:22"#;
        let tokenizer = Tokenizer::new(&mut string.chars());

        assert!(tokenizer.is_err());

        assert!(tokenizer.unwrap_err().message.contains("not allowed"));
    }

    #[test]
    fn test_correcly_tokenize_json() {
        let string = r#"{"test": {"test_inner": 1, "test_inner2": null}, "test1":"test",
        "test_after_return": true
        }"#;

        let tokenizer = Tokenizer::new(&mut string.chars());

        assert!(tokenizer.is_ok());

        let tokens = tokenizer.unwrap().tokens;

        let tokens = tokens
            .iter()
            .filter(|item| !matches!(item, Token::Whitespace))
            .collect::<Vec<_>>();

        assert_eq!(tokens.len(), 21);
    }

    #[test]
    fn test_return_error_if_not_parsed_correctly() {
        let string = r#"{"test: {"test_inner": 1, "test_inner2": null}, "test1":"test",
        "test_after_return": true
        }"#;

        let tokenizer = Tokenizer::new(&mut string.chars());

        assert!(tokenizer.is_err());

        assert!(matches!(tokenizer, Err(ParsingError { .. })));
    }

    #[test]
    fn test_escape_strings() {
        let string = r#""test\"_something""#;
        println!("{}", string);

        let tokenizer = Tokenizer::new(&mut string.chars());

        assert!(tokenizer.is_ok());

        let tokens = tokenizer.unwrap().tokens;

        let tokens = tokens
            .iter()
            .filter(|item| !matches!(item, Token::Whitespace))
            .collect::<Vec<_>>();

        assert_eq!(tokens.len(), 1);

        assert!(matches!(tokens[0], Token::StringValue(..)));
        let value = if let Token::StringValue(val) = &tokens[0] {
            val
        } else {
            panic!("This value must be a string, we asserted it one line above");
        };
        let expected = r#"test\"_something"#.to_string();
        assert_eq!(*value, expected);
    }
}
