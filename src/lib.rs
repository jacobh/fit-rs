#[macro_use]
extern crate error_chain;

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
}
