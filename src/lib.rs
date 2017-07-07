use std::io::Read;
use std::fs::File;
use std::path::Path;

pub struct FitFile {
    bytes: Vec<u8>,
}
impl FitFile {
    pub fn open<P: AsRef<Path>>(path: P) -> FitFile {
        FitFile {
            bytes: {
                let mut buf = vec![];
                let mut f = File::open(path).unwrap();
                f.read_to_end(&mut buf).unwrap();
                buf
            },
        }
    }
}
