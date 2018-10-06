///
/// Blitz Money
///
/// Module for manange storage of all data
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use std::error::Error;
use std::fs::{ File, OpenOptions };
use std::io::{ Read, Write };
use std::path::Path;
use std::option::Option;
use std::sync::Mutex;
use json::{ parse, JsonValue };

// Representation of storage
pub struct Storage {
    path_str: String,
    file: Option<File>,
    lines: Vec<String>,
}

// Representation of section data
pub struct Data<'a> {
    section: String,
    storage: &'a mut Storage,
    need_find_section: bool,
    last_position: usize
}

pub trait Model {

    // For set data into struct
    fn new(row: JsonValue) -> Self;
}

//
// Storage of bliz money is based in a single file. To
// ensure the integrity of data we need to centralize the
// access to file.
//
// More informations about the file storage into example.bms
//
lazy_static! {
    pub static ref LOCKED_STORAGE: Mutex<Storage> = Mutex::new(Storage { path_str: "/tmp/bmoneytmp.bms".to_string(), file: None, lines: Vec::new() });
}

impl Storage {

    // Open, or reopen, the file for storage. Create a file for store all data, if does not alred exists
    fn reopen_file(&mut self) {

        let path = Path::new(&self.path_str);

        // Open, or create, the file
        if self.file.is_none() {

            self.file = match OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(true)
                .open(path)
            {
                Ok(file) => Some(file),
                Err(e) => panic!("Couldn't create/open the storage file at {}", e.description()),
            };

            // Read lines from file
            let mut buff = String::new();
            if self.file.as_ref().unwrap().read_to_string(&mut buff).is_err() {
                panic!("Couldn't read lines of the storage file");
            }

            self.lines = Vec::new();
            for line in buff.lines() {
                self.lines.push(line.to_string());
            }
        }
    }

    // Creates a section of data into storage, if does not alread exists
    pub fn start_section(&mut self, name: String) -> bool {

        let mut has_write = false;

        if !self.check_section(name.clone()) {

            self.reopen_file();

            self.file.as_ref().unwrap().write_fmt(format_args!("::section::{}\n", name))
                .expect("Couldn't create the section on storage file");

            has_write = true;
        }

        if has_write {
            // Force reopen the file on next read
            self.file = None;
        }

        true
    }

    // Check if section exists
    pub fn check_section(&mut self, name: String) -> bool {

        self.reopen_file();

        for line in self.lines.clone() {
            if line == format!("::section::{}", name) {
                return true;
            }
        }

        false
    }

    // Return struct for read the data of the section
    pub fn get_section_data(&mut self, name: String) -> Data {
        Data { section: name, storage: self, last_position: 0, need_find_section: true }
    }
}

impl<'a> Data<'a> {

    // Find and adjust the value of position of section
    fn find_section(&mut self) {

        if self.need_find_section {
            // when other section use the buffer we need
            // reset the count of rows of the section

            self.need_find_section = false;

            for (i, line) in self.storage.lines.iter().enumerate() {
                if line.to_owned() == format!("::section::{}", self.section) {
                    self.last_position = i.clone();
                    break;
                }
            }
        }
    }

    // Return the next row of values into a struct filled
    pub fn next<M: Model>(&mut self) -> Result<M, &'static str> {

        self.storage.reopen_file();

        self.find_section();

        self.last_position = (self.last_position + 1).to_owned();

        if self.last_position.to_owned() < self.storage.lines.len() {

            let linestr = &self.storage.lines[self.last_position.to_owned()].trim();

            if !linestr.is_empty() {

                let row = match parse(&linestr) {
                    Ok(row) => row,
                    Err(e) => panic!("Couldn't parse the row: {}", e.description())
                };

                return Ok(M::new(row));
            }
        }

        Err("Next row not found")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_section() {
        let mut st = Storage { path_str: "/tmp/tscs".to_string(), file: None, lines: Vec::new() };
        assert!(!st.check_section("accounts".to_string()));
    }

    #[test]
    fn start_section() {
        let mut st = Storage { path_str: "/tmp/tssc".to_string(), file: None, lines: Vec::new() };
        assert!(st.start_section("accounts".to_string()));
    }
}
