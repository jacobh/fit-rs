#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

use nom::{le_u8, le_u32};

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
    pub fn parse_header(&self) -> Result<FitFileHeader> {
        Ok(fitfile_header(&self.bytes).unwrap().1)
    }
}

#[derive(Debug)]
pub struct FitFileHeader {
    header_size: u8,
    protocol_version: u8,
    profile_version: u8,
    data_size: u32,
}

named!(fitfile_header <FitFileHeader>, do_parse!(
    header_size: le_u8      >>
    protocol_version: le_u8 >>
    profile_version: le_u8  >>
    data_size: le_u32       >>
    // tag!(".FIT")            >>
    (
        FitFileHeader {
            header_size: header_size,
            protocol_version: protocol_version,
            profile_version: profile_version,
            data_size: data_size,
        }
    )
));
