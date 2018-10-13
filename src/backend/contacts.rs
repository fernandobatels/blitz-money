///
/// Blitz Money
///
/// Backend of module for manange contacts
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use backend::storage::*;
use json::JsonValue;

#[derive(Default, Clone, Debug)]
pub struct Contact {
   pub uuid: String,
   pub name: String,
   pub city_location: String,
}

impl Model for Contact {

    fn new(row: JsonValue, uuid: String) -> Contact {

        if row["name"].is_null() {
            panic!("Name not found into a row(id {}) contact", uuid);
        }

        if row["city_location"].is_null() {
            panic!("City location not found into a row(id {}) contact", uuid);
        }

        Contact {
            uuid: uuid,
            name: row["name"].to_string(),
            city_location: row["city_location"].to_string(),
        }
    }

    fn to_save(self) -> (String, bool, JsonValue) {

        (self.uuid.clone(), self.uuid.is_empty(), object!{
            "name" => self.name,
            "city_location" => self.city_location,
        })
    }
}

impl Contact {

    // Return a list with all contacts
    pub fn get_contacts(storage: &mut Storage) -> Vec<Contact> {

        storage.start_section("contacts".to_string());

        let mut data = storage.get_section_data("contacts".to_string());

        let mut list: Vec<Contact> = vec![];

        while let Ok(line) = data.next::<Contact>() {
            list.push(line);
        }

        return list;
    }

    // Return the contact of id
    pub fn get_contact(storage: &mut Storage, uuid: String) -> Result<Contact, &'static str> {

        storage.start_section("contacts".to_string());

        let mut data = storage.get_section_data("contacts".to_string());

        if data.find_by_id(uuid) {
            return data.next::<Contact>();
        }

        Err("Contact not found")
    }

    // Save updates, or create new, contact on storage
    pub fn store_contact(storage: &mut Storage, contact: Contact) {

        storage.start_section("contacts".to_string());

        let mut data = storage.get_section_data("contacts".to_string());

        data.save(contact);
    }

    // Remvoe contact of storage
    pub fn remove_contact(storage: &mut Storage, uuid: String) {

        storage.start_section("contacts".to_string());

        let mut data = storage.get_section_data("contacts".to_string());

        data.remove_by_id(uuid);
    }
}
