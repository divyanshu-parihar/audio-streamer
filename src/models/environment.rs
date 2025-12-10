use log::{error, info};
use std::fs::File;
enum BinaryType {
    SERVER,
    CLIENT,
}
pub struct Environment<'a> {
    pub file_name: &'a String,
    pub server: BinaryType,
}

impl<'a> Environment<'a> {
    pub fn new(name: &'a String, is_server: bool) -> Self {
        // check for conditions
        let result = File::open(&name);

        let _ = match result {
            Ok(f) => {
                info!("File Verified Exists {:?}", f.metadata())
            }
            Err(e) => {
                error!("File not found / Something Else {e}");
                panic!()
            }
        };
        if (name.len() > 0) {
            Environment {
                file_name: &name,
                server: if is_server {
                    BinaryType::SERVER
                } else {
                    BinaryType::CLIENT
                },
            }
        } else {
            panic!("checks not passed");
        }
    }
}
