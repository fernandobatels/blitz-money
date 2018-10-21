///
/// Blitz Money
///
/// Frontend/Ui of module for manange contacts of user
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use backend::contacts::Contact;
use backend::storage::Storage;
use ui::ui::*;

pub struct Contacts {}

impl Contacts {

    // List of user contacts
    pub fn list(mut storage: Storage, _params: Vec<String>, is_csv: bool) {

        let contacts = Contact::get_contacts(&mut storage);
        let mut table = Output::new_table();

        table.set_titles(row![b->"Name", b->"City", b->"#id"]);

        for contact in contacts {

            table.add_row(row![
                contact.name,
                contact.city_location,
                contact.uuid
            ]);
        }

        Output::print_table(table, is_csv);
    }

    // Create new contact
    pub fn add(mut storage: Storage, params: Vec<String>) {

        if params.len() == 2 {
            // Shell mode

            let name = Input::param("Contact name".to_string(), true, params.clone(), 0);
            let city = Input::param("City".to_string(), true, params.clone(), 1);

            Contact::store_contact(&mut storage, Contact {
                uuid: "".to_string(),
                city_location: city,
                name: name,
            });
        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let name = Input::read("Contact name".to_string(), true, None);
            let city = Input::read("City".to_string(), true, None);

            Contact::store_contact(&mut storage, Contact {
                uuid: "".to_string(),
                city_location: city,
                name: name,
            });
        } else {
            // Help mode
            println!("How to use: bmoney contacts add [name] [city]");
            println!("Or with interactive mode: bmoney contacts add -i")
        }
    }

    // Update a existing contact
    pub fn update(mut storage: Storage, params: Vec<String>) {

        if params.len() == 3 {
            // Shell mode

            let mut contact = Contact::get_contact(&mut storage, params[0].trim().to_string())
                .expect("Contact not found");

            if params[1] == "name" {
                contact.name = Input::param("Contact name".to_string(), true, params.clone(), 2);
            } else if params[1] == "city" {
                contact.city_location = Input::param("City".to_string(), true, params.clone(), 2);
            } else {
                panic!("Field not found!");
            }

            Contact::store_contact(&mut storage, contact);

        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let id = Input::read("Contact id".to_string(), true, None);

            let mut contact = Contact::get_contact(&mut storage, id)
                .expect("Contact not found");

            contact.name = Input::read("Contact name".to_string(), true, Some(contact.name));
            contact.city_location = Input::read("City".to_string(), true, Some(contact.city_location));

            Contact::store_contact(&mut storage, contact);
        } else {
            // Help mode
            println!("How to use: bmoney contacts update [id] [name|city_location] [value]");
            println!("Or with interactive mode: bmoney contacts update -i")
        }
    }

    // Remove a existing contact
    pub fn rm(mut storage: Storage, params: Vec<String>) {

        if params.len() == 1 {
            // Shell mode

            Contact::remove_contact(&mut storage, params[0].trim().to_string());
        } else {
            // Help mode
            println!("How to use: bmoney contacts rm [id]");
        }
    }
}
