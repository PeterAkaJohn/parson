use std::io::{BufRead, BufReader};

use crate::{ParsingError, ParsonResult};

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Int(i64),
    Float(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    String(String),
    Number(Number),
    Null,
    Boolean(bool),
    Comma,
}

impl TryFrom<f64> for Token {
    type Error = ParsingError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let value = if value.fract() == 0.0 {
            Token::Number(Number::Int(value as i64))
        } else {
            Token::Number(Number::Float(value))
        };
        Ok(value)
    }
}

pub struct Tokenizer {
    pub tokens: Vec<Vec<Token>>,
}

#[derive(Debug)]
struct Line(Vec<Token>);

impl TryFrom<String> for Line {
    type Error = ParsingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut tokens: Vec<Token> = vec![];
        let mut iterator = value.chars().peekable();
        while let Some(char) = iterator.peek() {
            match char {
                '0'..='9' => {
                    //parse as number
                    //parse until comma
                    let mut number_string = String::new();
                    for digit in iterator.by_ref() {
                        if digit == ',' {
                            break;
                        }
                        number_string.push(digit);
                    }

                    let token: Token = number_string
                        .parse::<f64>()
                        .map_err(|_| ParsingError {
                            message: format!("field with value {number_string} is not a number"),
                        })?
                        .try_into()?;
                    tokens.push(token);
                }
                '"' => {
                    //parse as string but can have escaped " with double ""

                    iterator.next();
                    let mut value = String::new();
                    while let Some(inner_char) = iterator.next() {
                        if inner_char == '"' {
                            //peek the next
                            if let Some('"') = iterator.peek() {
                                iterator.next();
                            } else {
                                iterator.next();
                                break;
                            }
                        }
                        value.push(inner_char);
                    }
                    let token = if &value == "true" || &value == "false" {
                        Token::Boolean(value == "true")
                    } else {
                        Token::String(value)
                    };
                    tokens.push(token);
                }
                _ => {
                    let mut string_value = String::new();
                    for inner_char in iterator.by_ref() {
                        if inner_char == ',' {
                            break;
                        }
                        string_value.push(inner_char);
                    }

                    let token = if &string_value == "true" || &string_value == "false" {
                        Token::Boolean(string_value == "true")
                    } else {
                        Token::String(string_value)
                    };
                    tokens.push(token);
                }
            }
        }

        Ok(Line(tokens))
    }
}

impl Tokenizer {
    pub fn new(reader: BufReader<&[u8]>) -> ParsonResult<Self> {
        Ok(Self {
            tokens: Self::parse_tokens(reader)?,
        })
    }

    fn parse_tokens(reader: BufReader<&[u8]>) -> ParsonResult<Vec<Vec<Token>>> {
        let mut value = vec![];

        let mut line_length = 0;

        for (idx, line) in reader.lines().enumerate() {
            let line = line.map_err(|_| ParsingError {
                message: "failed to tokenize buf".to_string(),
            })?;
            let line: Line = line.try_into()?;
            if idx == 0 {
                line_length = line.0.len()
            }

            if idx != 0 && line.0.len() != line_length {
                return Err(ParsingError {
                    message: format!("lines do not have the same number of columns {line_length}"),
                });
            }

            value.push(line.0);
        }

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::csv::token::{Number, Token};

    use super::Tokenizer;

    #[test]
    fn fail_tokenizing_uneven_number_of_columns() {
        let csv_string = "test1,test2,num1,num2,additional,\nval1,val2,2,3.4";
        let tokens = Tokenizer::parse_tokens(BufReader::new(csv_string.as_bytes()));
        assert!(tokens.is_err());
        assert!(tokens
            .unwrap_err()
            .message
            .contains("do not have the same number of columns"));
    }

    #[test]
    fn tokenize_escaped_string() {
        let csv_string = "test1,test2,num1,num2\n\"val1\",val2,2,3.4";
        let tokens = Tokenizer::parse_tokens(BufReader::new(csv_string.as_bytes()));
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        assert_eq!(tokens.len(), 2);
        let token = tokens[1][0].clone();
        let token_value = if let Token::String(t) = token {
            t
        } else {
            "".to_string()
        };
        assert_eq!(token_value, "val1".to_string());

        let csv_string = "test1,test2,num1,num2\n\"va\"\"l1\",val2,2,3.4";
        let tokens = Tokenizer::parse_tokens(BufReader::new(csv_string.as_bytes()));
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        assert_eq!(tokens.len(), 2);
        let token = tokens[1][0].clone();
        let token_value = if let Token::String(t) = token {
            t
        } else {
            "".to_string()
        };
        assert_eq!(token_value, "va\"l1".to_string());
    }

    #[test]
    fn tokenize_bool() {
        let csv_string = "test1,test2,num1,num2,condition\n\"val1\",val2,2,3.4,true";
        let tokens = Tokenizer::parse_tokens(BufReader::new(csv_string.as_bytes()));
        assert!(tokens.clone().is_ok_and(|tok| tok.len() == 2));
        let tokens = tokens.unwrap();
        let token = tokens[1][4].clone();
        assert!(matches!(token, Token::Boolean(true)));

        let csv_string = "test1,test2,num1,num2,condition\n\"val1\",val2,2,3.4,\"true\"";
        let tokens = Tokenizer::parse_tokens(BufReader::new(csv_string.as_bytes()));
        assert!(tokens.clone().is_ok_and(|tok| tok.len() == 2));
        let tokens = tokens.unwrap();
        let token = tokens[1][4].clone();
        assert!(matches!(token, Token::Boolean(true)))
    }

    #[test]
    fn tokenize_correctly() {
        let csv_string = "test1,test2,num1,num2\nval1,val2,2,3.4";
        let tokens = Tokenizer::parse_tokens(BufReader::new(csv_string.as_bytes()));
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        assert_eq!(tokens.len(), 2);

        let first_line = tokens.first().unwrap();
        let expected = vec![
            Token::String("test1".to_string()),
            Token::String("test2".to_string()),
            Token::String("num1".to_string()),
            Token::String("num2".to_string()),
        ];
        assert_eq!(*first_line, expected);
        let second_line = tokens.get(1).unwrap();
        let expected = vec![
            Token::String("val1".to_string()),
            Token::String("val2".to_string()),
            Token::Number(Number::Int(2)),
            Token::Number(Number::Float(3.4)),
        ];
        assert_eq!(*second_line, expected);
    }
}
