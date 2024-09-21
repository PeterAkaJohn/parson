mod token;
mod value;

use std::{collections::HashMap, io::BufReader};

use token::{Token, Tokenizer};
pub use value::Value;

use crate::{ParsingError, ParsonResult};

pub struct CsvParser {
    tokenizer: Tokenizer,
}

impl TryFrom<&Token> for String {
    type Error = ParsingError;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value {
            Token::String(string) => Ok(string.to_string()),
            _ => Err(ParsingError {
                message: format!("token {:#?} is not a string and it should be", value),
            }),
        }
    }
}

impl CsvParser {
    pub fn new(buf: &[u8]) -> ParsonResult<Self> {
        let bufread = BufReader::new(buf);
        Ok(Self {
            tokenizer: Tokenizer::new(bufread)?,
        })
    }

    pub fn parse(&self) -> ParsonResult<Vec<HashMap<String, Value>>> {
        let mut value = vec![];

        let mut header: Vec<String> = vec![];

        for (idx, line) in self.tokenizer.tokens.iter().enumerate() {
            match idx {
                0 => {
                    // this is the header
                    // all row items must be strings
                    if line.iter().any(|item| !matches!(*item, Token::String(..))) {
                        return Err(ParsingError {
                            message: "header items must all be strings".to_string(),
                        });
                    }
                    header = line
                        .iter()
                        .filter(|t| matches!(t, Token::String(..)))
                        .map(|val| val.try_into())
                        .collect::<ParsonResult<Vec<String>>>()?;
                }
                _ => {
                    let row_values = line
                        .iter()
                        .zip(&header)
                        .map(|(row, col)| (col.to_string(), row.into()))
                        .collect::<HashMap<String, Value>>();

                    value.push(row_values);
                }
            };
        }

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::CsvParser;

    #[test]
    fn parsing_correctly() {
        let csv_string = "test1,test2\nvaltest1,valtest2\nvaltest1,valtest2";

        let parser = CsvParser::new(csv_string.as_bytes());

        assert!(parser.is_ok());

        let parser = parser.unwrap();

        let parsedcsv = parser.parse();

        assert!(parsedcsv.is_ok());

        assert_eq!(parsedcsv.unwrap().iter().len(), 2);
    }
}
