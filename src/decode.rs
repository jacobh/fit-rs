use std::os::raw::c_void;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use ffi;

enum FitBool {
    True = 1,
    False = 0
}

pub struct Message {
    data: u8,
    num: u16,
}

pub fn decode<P: AsRef<Path>>(path: P) -> Vec<Message> {
    let mut reader = {
        // let mut buf = vec![];
        let f = File::open(path).unwrap();
        BufReader::new(f)
        // f.read_to_end(&mut buf).unwrap();
        // buf
    };
    
    let mut state: ffi::FIT_CONVERT_STATE;

    unsafe {
        state = ::std::mem::uninitialized();
        ffi::FitConvert_Init(&mut state as *mut ffi::FIT_CONVERT_STATE, FitBool::False as u8);
    }

    let mut messages: Vec<Message> = vec![];

    loop {
        let mut buf = [0; 8];
        match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                unsafe {
                    let return_val = ffi::FitConvert_Read(&mut state as *mut ffi::FIT_CONVERT_STATE, buf.as_ptr() as *const c_void, n as u32);
                    match return_val {
                        ffi::FIT_CONVERT_RETURN::FIT_CONVERT_CONTINUE => continue,
                        ffi::FIT_CONVERT_RETURN::FIT_CONVERT_MESSAGE_AVAILABLE => {
                            println!("message available!");
                            messages.push(Message{
                                data: *ffi::FitConvert_GetMessageData(&mut state as *mut ffi::FIT_CONVERT_STATE),
                                num: ffi::FitConvert_GetMessageNumber(&mut state as *mut ffi::FIT_CONVERT_STATE),
                            });
                        },
                        ffi::FIT_CONVERT_RETURN::FIT_CONVERT_MESSAGE_NUMBER_FOUND => unimplemented!(),
                        ffi::FIT_CONVERT_RETURN::FIT_CONVERT_ERROR => panic!(),
                        ffi::FIT_CONVERT_RETURN::FIT_CONVERT_END_OF_FILE => break,
                        ffi::FIT_CONVERT_RETURN::FIT_CONVERT_PROTOCOL_VERSION_NOT_SUPPORTED => panic!("Protocol version not supported"),
                        ffi::FIT_CONVERT_RETURN::FIT_CONVERT_DATA_TYPE_NOT_SUPPORTED => panic!("Data type not supported"),
                        // _ => unimplemented!(),
                    }
                }
            }
            _ => panic!()
        }
    }
    messages
}
