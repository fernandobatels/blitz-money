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

    fn new(row: JsonValue, uuid: String, _storage: &mut Storage, _can_recursive: bool) -> Contact {

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

#[cfg(test)]
mod tests {

    use super::*;
    use uuid::Uuid;

    fn populate() -> String {

        let path = "/tmp/bmoney-".to_owned() + &Uuid::new_v4().to_string();

        let mut st = Storage { path_str: path.clone(), file: None, lines: Vec::new() };

        assert!(st.start_section("contacts".to_string()));

        let mut data = st.get_section_data("contacts".to_string());

        data.save(Contact { uuid: "".to_string(), name: "contact 1".to_string(), city_location: "city A".to_string() });
        data.save(Contact { uuid: "".to_string(), name: "contact 2".to_string(), city_location: "city B".to_string() });
        data.save(Contact { uuid: "".to_string(), name: "contact 3".to_string(), city_location: "city C".to_string() });
        data.save(Contact { uuid: "".to_string(), name: "contact 4".to_string(), city_location: "city D".to_string() });

        path
    }

    #[test]
    fn get_contacts() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new() };

        let contacts = Contact::get_contacts(&mut st);

        assert_eq!(contacts[0].name, "contact 4".to_string());
        assert_eq!(contacts[0].city_location, "city D".to_string());
        assert_eq!(contacts[1].name, "contact 3".to_string());
        assert_eq!(contacts[1].city_location, "city C".to_string());
        assert_eq!(contacts[2].name, "contact 2".to_string());
        assert_eq!(contacts[2].city_location, "city B".to_string());
        assert_eq!(contacts[3].name, "contact 1".to_string());
        assert_eq!(contacts[3].city_location, "city A".to_string());
    }

    #[test]
    fn get_contact() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new() };

        let contacts = Contact::get_contacts(&mut st);

        let uuid = contacts[0].uuid.clone();

        let contact = Contact::get_contact(&mut st, uuid);

        assert!(contact.is_ok());
        assert_eq!(contact.clone().unwrap().name, "contact 4".to_string());
        assert_eq!(contact.unwrap().city_location, "city D".to_string());

        let contacte = Contact::get_contact(&mut st, "NOOOO".to_string());

        assert!(contacte.is_err());
    }


    #[test]
    fn store_contact() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new() };

        Contact::store_contact(&mut st, Contact { uuid: "".to_string(), name: "contact 5".to_string(), city_location: "city E".to_string() });

        let contacts = Contact::get_contacts(&mut st);

        assert_eq!(contacts[0].name, "contact 5".to_string());
        assert_eq!(contacts[0].city_location, "city E".to_string());
        assert_eq!(contacts[1].name, "contact 4".to_string());
        assert_eq!(contacts[1].city_location, "city D".to_string());
    }

    #[test]
    fn remove_contact() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new() };

        let contacts = Contact::get_contacts(&mut st);

        let uuid = contacts[0].uuid.clone();

        let contact = Contact::get_contact(&mut st, uuid.clone());

        assert!(contact.is_ok());

        Contact::remove_contact(&mut st, uuid.clone());

        let contacte = Contact::get_contact(&mut st, uuid.clone());

        assert!(contacte.is_err());
    }
}
