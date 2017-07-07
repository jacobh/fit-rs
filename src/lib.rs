#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

use nom::{le_u8, le_u16, le_u32};

use std::io::Read;
use std::fs::File;
use std::path::Path;

mod errors;

use errors::*;

pub struct FitFile {
    bytes: Vec<u8>,
}
impl FitFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<FitFile> {
        Ok(FitFile {
            bytes: {
                let mut buf = vec![];
                let mut f = File::open(path)?;
                f.read_to_end(&mut buf)?;
                buf
            },
        })
    }
    pub fn get_header(&self) -> Result<FitFileHeader> {
        fitfile_header(&self.bytes)
            .to_result()
            .map_err(|e| e.description().into())
    }
    pub fn validate_data(&self) -> Result<()> {
        let header = self.get_header()?;
        let data_size = self.bytes.len() - header.header_size as usize - 2;
        match data_size == header.data_size as usize {
            true => Ok(()),
            false => Err("Data looks to be corrupted".into()),
        }
    }
}

#[derive(Debug)]
pub struct FitFileHeader {
    header_size: u8,
    protocol_version: u8,
    profile_version: u16,
    data_size: u32,
    crc: u16,
}

named!(fitfile_header <FitFileHeader>, do_parse!(
    header_size: le_u8      >>
    protocol_version: le_u8 >>
    profile_version: le_u16 >>
    data_size: le_u32       >>
    tag!(".FIT")            >>
    crc: le_u16             >>
    (
        FitFileHeader {
            header_size: header_size,
            protocol_version: protocol_version,
            profile_version: profile_version,
            data_size: data_size,
            crc: crc,
        }
    )
));
