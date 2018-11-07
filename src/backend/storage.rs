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
use json::{ parse, JsonValue };
use uuid::Uuid;

// Representation of storage
pub struct Storage {
    pub path_str: String,
    pub file: Option<File>,
    pub lines: Vec<String>,
}

// Representation of section data
pub struct Data<'a> {
    section: String,
    storage: &'a mut Storage,
    need_find_section: bool,
    last_position: usize,
    pub can_recursive: bool
}

pub trait Model {

    // For set data into struct
    fn new(row: JsonValue, uuid: String, storage: &mut Storage, can_recursive: bool) -> Self;

    // Parse to storage data
    fn to_save(self) -> (String, bool, JsonValue);
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

            self.file.as_ref().unwrap().write_fmt(format_args!("\n::section::{}\n", name))
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
        Data { section: name, storage: self, last_position: 0, need_find_section: true, can_recursive: true }
    }

    // Persist current lines on storage
    pub fn persist(&mut self) {

        if self.file.as_ref().unwrap().set_len(0).is_err() {
            panic!("Couldn't reset storage file");
        }

        for line in self.lines.clone() {
            self.file.as_ref().unwrap()
                .write_fmt(format_args!("{}\n", line))
                .expect("Couldn't write line on storage file");
        }

        // Force reopen the file on next read
        // self.storage.file = None;
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

    // Find the next row by the id. After this method is necessary
    // use the next() method for get the row
    pub fn find_by_id(&mut self, uuid: String) -> bool {

        self.storage.reopen_file();
        // To force postion after section start
        self.need_find_section = true;
        self.last_position = 0;
        self.find_section();

        let mut first = true;

        for (i, line) in self.storage.lines.iter().enumerate() {

            // We start the find only after the start of section
            if line.trim().is_empty() || self.last_position > i {
                continue;
            }

            if !first && line.trim().chars().take(11).collect::<String>() == "::section::" {
                // And stop on next section
                return false;
            }

            if line.trim().chars().take(36).collect::<String>() == uuid {
                self.last_position = i.clone() - 1;
                return true;
            }

            first = false;
        }

        false
    }

    // Return the next row of values into a struct filled
    pub fn next<M: Model>(&mut self) -> Result<M, &'static str> {

        self.storage.reopen_file();

        self.find_section();

        self.last_position = (self.last_position + 1).to_owned();

        if self.last_position.to_owned() < self.storage.lines.len() {

            let linestr = self.storage.lines[self.last_position.to_owned()].clone();

            if !linestr.trim().is_empty() {

                let uuid = linestr.chars().take(36).collect();
                let json = linestr.chars().skip(37).collect::<String>();

                let row = match parse(&json) {
                    Ok(row) => row,
                    Err(e) => panic!("Couldn't parse the row: {}", e.description())
                };

                return Ok(M::new(row, uuid, self.storage, self.can_recursive));
            }
        }

        Err("Next row not found")
    }

    // Insert, or update, the row into storage and return the uuid
    pub fn save<M: Model>(&mut self, row: M) -> String {

        let (mut uuid, is_new, data) = row.to_save();

        self.storage.reopen_file();

        if is_new {
            // New register

            // To force postion after section start
            self.need_find_section = true;
            self.find_section();

            uuid = Uuid::new_v4().to_string();
            self.storage.lines.insert(self.last_position + 1, format!("{} {}", uuid, data.dump()));

            self.need_find_section = true;
        } else {
            // Register to update
            for (i, line) in self.storage.lines.clone().iter().enumerate() {
                if line.chars().take(36).collect::<String>() == uuid {
                    self.storage.lines[i] = format!("{} {}", uuid, data.dump());
                    break;
                }
            }
        }

        self.storage.persist();

        uuid
    }

    // Remove row of storage by id
    pub fn remove_by_id(&mut self, uuid: String) {

        self.storage.reopen_file();

        for (i, line) in self.storage.lines.clone().iter().enumerate() {
            if line.chars().take(36).collect::<String>() == uuid {
                self.storage.lines.remove(i);
                break;
            }
        }

        self.storage.persist();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use uuid::Uuid;

    #[derive(Default, Clone, Debug)]
    pub struct TestModel {
       pub uuid: String,
       pub name: String,
    }

    impl Model for TestModel {

        fn new(row: JsonValue, uuid: String, _storage: &mut Storage, _can_recursive: bool) -> TestModel {

            if row["name"].is_null() {
                panic!("Name not found into a row(id {})", uuid);
            }

            TestModel {
                uuid: uuid,
                name: row["name"].to_string(),
            }
        }

        fn to_save(self) -> (String, bool, JsonValue) {
            (self.uuid.clone(), self.uuid.is_empty(), object!{
                "name" => self.name,
            })
        }
    }

    #[test]
    fn check_section() {

        let mut st = Storage { path_str: "/tmp/bmoney-".to_owned() + &Uuid::new_v4().to_string(), file: None, lines: Vec::new() };
        assert!(!st.check_section("accounts".to_string()));
    }

    #[test]
    fn start_section() {

        let mut st = Storage { path_str: "/tmp/bmoney-".to_owned() + &Uuid::new_v4().to_string(), file: None, lines: Vec::new() };
        assert!(st.start_section("accounts".to_string()));
    }

    #[test]
    fn get_section_data() {

        let mut st = Storage { path_str: "/tmp/bmoney-".to_owned() + &Uuid::new_v4().to_string(), file: None, lines: Vec::new() };

        assert!(st.start_section("accounts".to_string()));

        let data = st.get_section_data("accounts".to_string());

        assert_eq!(data.section, "accounts".to_string());
        assert_eq!(data.last_position, 0);
        assert_eq!(data.need_find_section, true);
        assert_eq!(data.can_recursive, true);
    }

    #[test]
    fn save() {

        let mut st = Storage { path_str: "/tmp/bmoney-".to_owned() + &Uuid::new_v4().to_string(), file: None, lines: Vec::new() };

        assert!(st.start_section("accounts".to_string()));

        let mut data = st.get_section_data("accounts".to_string());

        let test = TestModel { uuid: "".to_string(), name: "TESTTT!".to_string() };

        let new_uuid = data.save(test);

        // Valid uuid
        assert_eq!(new_uuid.len(), 36);

        let test2 = TestModel { uuid: new_uuid.clone(), name: "TEST UPDATE!".to_string() };

        let updated_uuid = data.save(test2);

        // If is the same uuid
        assert_eq!(new_uuid, updated_uuid);
    }

    #[test]
    fn next() {

        let path = "/tmp/bmoney-".to_owned() + &Uuid::new_v4().to_string();

        let mut st = Storage { path_str: path.clone(), file: None, lines: Vec::new() };

        assert!(st.start_section("accounts".to_string()));

        let mut data = st.get_section_data("accounts".to_string());

        data.save(TestModel { uuid: "".to_string(), name: "FIND ME!".to_string() });

        assert!(data.next::<TestModel>().is_ok());

        // Load again
        let mut st2 = Storage { path_str: path, file: None, lines: Vec::new() };

        let mut data2 = st2.get_section_data("accounts".to_string());

        let row = data2.next::<TestModel>();

        assert!(row.is_ok());

        assert_eq!(row.clone().unwrap().uuid.len(), 36);
        assert_eq!(row.unwrap().name, "FIND ME!".to_string());
    }

    #[test]
    fn find_by_id() {

        let mut st = Storage { path_str: "/tmp/bmoney-".to_owned() + &Uuid::new_v4().to_string(), file: None, lines: Vec::new() };

        assert!(st.start_section("accounts".to_string()));

        let mut data = st.get_section_data("accounts".to_string());

        data.save(TestModel { uuid: "".to_string(), name: "FIND ME!".to_string() });
        let new_uuid = data.save(TestModel { uuid: "".to_string(), name: "FIND ME2!".to_string() });
        data.save(TestModel { uuid: "".to_string(), name: "FIND ME3!".to_string() });

        // Valid uuid
        assert_eq!(new_uuid.len(), 36);

        // If is finded
        assert!(data.find_by_id(new_uuid.clone()));

        let row = data.next::<TestModel>();

        assert!(row.is_ok());

        assert_eq!(row.clone().unwrap().uuid, new_uuid);
        assert_eq!(row.unwrap().name, "FIND ME2!".to_string());
    }

    #[test]
    fn remove_by_id() {

        let mut st = Storage { path_str: "/tmp/bmoney-".to_owned() + &Uuid::new_v4().to_string(), file: None, lines: Vec::new() };

        assert!(st.start_section("accounts".to_string()));

        let mut data = st.get_section_data("accounts".to_string());

        data.save(TestModel { uuid: "".to_string(), name: "FIND ME!".to_string() });
        let new_uuid = data.save(TestModel { uuid: "".to_string(), name: "FIND ME2!".to_string() });
        data.save(TestModel { uuid: "".to_string(), name: "FIND ME3!".to_string() });

        // Valid uuid
        assert_eq!(new_uuid.len(), 36);

        // If is finded
        assert!(data.find_by_id(new_uuid.clone()));

        data.remove_by_id(new_uuid.clone());

        // uuid can't more finded
        assert!(!data.find_by_id(new_uuid));
    }
}
