mod token;
mod value;

use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

pub use value::Value;

use crate::{ParsingError, ParsonResult};

pub struct CsvParser<'a> {
    reader: BufReader<&'a [u8]>,
}

impl<'a> CsvParser<'a> {
    pub fn new(buf: &'a [u8]) -> ParsonResult<Self> {
        let bufread = BufReader::new(buf);
        Ok(Self { reader: bufread })
    }

    pub fn parse(self) -> ParsonResult<Vec<HashMap<String, Value>>> {
        let mut value = vec![];

        let mut header = vec![];

        for (idx, line) in self.reader.lines().enumerate() {
            let line = line.map_err(|_| ParsingError {
                message: "failed to read line when parsing json".to_string(),
            })?;

            match idx {
                0 => {
                    // this is the header
                    let splitvalue = line.split(',');
                    header = splitvalue.map(|val| val.to_string()).collect::<_>()
                }
                _ => {
                    let row_values = line
                        .split(',')
                        .zip(&header)
                        .map(|(value, col)| {
                            let value = if value.eq("null") {
                                Value::Null
                            } else if let Ok(val) = value.parse::<f64>() {
                                Value::Number(val)
                            } else if value == "true" || value == "false" {
                                Value::Boolean(value == "true")
                            } else {
                                Value::String(value.to_string())
                            };

                            (col.to_string(), value)
                        })
                        .collect::<HashMap<String, Value>>();

                    value.push(row_values);
                }
            };
        }

        println!("{:#?}", value);

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::CsvParser;

    #[test]
    fn parsing_correctly() {
        let csv_string = "test1,test2\n1valtest1,1valtest2\n2valtest1,2valtest2";

        let parser = CsvParser::new(csv_string.as_bytes());

        assert!(parser.is_ok());

        let parser = parser.unwrap();

        let parsedcsv = parser.parse();

        assert!(parsedcsv.is_ok());

        assert_eq!(parsedcsv.unwrap().iter().len(), 2);
    }
}
