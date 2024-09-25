use std::{
    io::{BufReader, Cursor, Read, Seek, SeekFrom},
    str::from_utf8,
};

use parquet::FileMetaData;
use thrift::protocol::{TCompactInputProtocol, TSerializable};

use crate::{ParsingError, ParsonResult};

mod parquet;

#[derive(Debug)]
pub struct ParqParser {
    parsed: (),
}

impl ParqParser {
    pub fn new(bytes: &[u8]) -> ParsonResult<Self> {
        let something = Self::parse_parq(bytes)?;
        Ok(Self { parsed: something })
    }

    fn is_parq_file(reader: &mut Cursor<&[u8]>) -> ParsonResult<()> {
        let mut last_four_bytes: [u8; 4] = [0; 4];
        let _ = reader
            .read(&mut last_four_bytes)
            .map_err(|_| ParsingError {
                message: "could not read magic number".to_string(),
            })?;

        let magic_number = from_utf8(&last_four_bytes).map_err(|_| ParsingError {
            message: "not a parq".to_string(),
        })?;

        match magic_number {
            "PAR1" => Ok(()),
            _ => Err(ParsingError {
                message: "Not a parq file".to_string(),
            }),
        }
    }

    fn get_footer_length(reader: &mut Cursor<&[u8]>) -> ParsonResult<u32> {
        let mut footer_length: [u8; 4] = [0; 4];
        let _ = reader.read(&mut footer_length);
        Ok(u32::from_le_bytes(footer_length))
    }

    fn get_file_metadata(
        reader: &mut Cursor<&[u8]>,
        footer_length: usize,
    ) -> ParsonResult<FileMetaData> {
        println!("{}", footer_length);
        let mut bytes: Vec<u8> = vec![0; footer_length];
        reader.read_exact(&mut bytes).map_err(|_| ParsingError {
            message: "failed to read file_metadata bytes".to_string(),
        })?;
        let mut protocol = TCompactInputProtocol::new(bytes.as_slice());
        let file_metadata =
            FileMetaData::read_from_in_protocol(&mut protocol).map_err(|_| ParsingError {
                message: "Failed to read parq metadata".to_string(),
            })?;
        Ok(file_metadata)
    }

    pub fn parse_parq(bytes: &[u8]) -> ParsonResult<()> {
        let mut reader = Cursor::new(bytes);
        let _ = reader.seek(SeekFrom::End(-8));
        let footer_length = Self::get_footer_length(&mut reader)?;
        Self::is_parq_file(&mut reader)?;

        let _ = reader.seek(SeekFrom::End(-8 - footer_length as i64));
        let file_metadata = Self::get_file_metadata(&mut reader, footer_length as usize)?;

        println!("{:?}", file_metadata);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::ParqParser;

    fn test_file_bytes() -> Vec<u8> {
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = format!("{}/{}", crate_dir, "/resources/test.parq");
        fs::read(path).unwrap()
    }

    #[test]
    fn created_correctly() {
        let bytes = test_file_bytes();

        let parser = ParqParser::new(&bytes);
        assert!(parser.is_ok());
    }
    #[test]
    fn not_parq_file() {
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = format!("{}/{}", crate_dir, "/resources/notaparq.parq");
        let bytes = fs::read(path).unwrap();

        let parser = ParqParser::new(&bytes);
        assert!(parser.is_err());
        assert!(parser.unwrap_err().message.contains("Not a parq"));
    }
}
