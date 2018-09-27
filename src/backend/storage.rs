///
/// Blitz Money
///
/// Module for manange storage of all data
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::option::Option;
use std::sync::Mutex;

pub struct Storage {
    path_str: String,
    file: Option<File>
}

//
// Storage of bliz money is based in a single file. To
// ensure the integrity of data we need to centralize the
// access to file.
//
lazy_static! {
    pub static ref LOCKED_STORAGE: Mutex<Storage> = Mutex::new(start_storage());
}

//
// Start the Storage struct for locked storage
//
fn start_storage() -> Storage {

    let mut st = Storage { path_str: "/tmp/bmoneytmp.bms".to_string(), file: None };

    st.init();

    st
}

impl Storage {

    // Create a file for store all data, if does not alred exists
    fn init(&mut self) {

        let path = Path::new(&self.path_str);

        self.file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
        {
            Err(e) => panic!("Couldn't create the storage file at {}", e.description()),
            Ok(file) => Some(file),
        };
    }

    // Creates a section of data into storage, if does not alread exists
    pub fn start_section(&self, name: String) -> bool {
        if !self.check_section(name) {
        }

        true
    }

    // Check if section exists
    pub fn check_section(&self, name: String) -> bool {

        let file = match &self.file {
            Some(file) => file,
            None => panic!("File of storage not opened")
        };

        for line in BufReader::new(file).lines() {
            println!("{}{:?}", name, line);
        }

        true
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_section() {
        let mut st = Storage { path_str: "/tmp/tscs", file: None };
        st.init();
        assert!(!st.check_section("accounts".to_string()));
    }

    #[test]
    fn start_section() {
        let mut st = Storage { path_str: "/tmp/tssc", file: None };
        st.init();
        assert!(st.start_section("accounts".to_string()));
    }
}
