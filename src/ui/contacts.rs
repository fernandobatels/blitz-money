///
/// Blitz Money
///
/// Frontend/Ui of module for manange contacts of user
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use prettytable::*;
use csv::*;
use std::io;

use backend::contacts::Contact;
use backend::storage::Storage;

pub struct ContactsUI {}

impl ContactsUI {

    // List of user contacts
    pub fn list(mut storage: Storage, _params: Vec<String>, is_csv: bool) {

        let contacts = Contact::get_contacts(&mut storage);
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        table.set_titles(row![b->"Name", b->"City", b->"#id"]);

        for contact in contacts {

            table.add_row(row![
                contact.name,
                contact.city_location,
                contact.uuid
            ]);
        }


        if is_csv {
             let mut wtr = WriterBuilder::new()
                .quote_style(QuoteStyle::NonNumeric)
                .from_writer(io::stdout());

            table.to_csv_writer(wtr);
        } else {
            table.printstd();
        }
    }

    // Create new contact
    pub fn add(mut storage: Storage, params: Vec<String>) {

        if params.len() == 2 {
            // Shell mode

            Contact::store_contact(&mut storage, Contact {
                uuid: "".to_string(),
                city_location: params[1].trim().to_string(),
                name: params[0].trim().to_string(),
            });
        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            println!("Contact name:");
            let mut name = String::new();
            io::stdin().read_line(&mut name)
                .expect("Failed to read name");

            println!("City:");
            let mut city_location = String::new();
            io::stdin().read_line(&mut city_location)
                .expect("Failed to read city");

            Contact::store_contact(&mut storage, Contact {
                uuid: "".to_string(),
                city_location: city_location.trim().to_string(),
                name: name.trim().to_string(),
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
                contact.name = params[2].trim().to_string();
            } else if params[1] == "city" {
                contact.city_location = params[2].trim().to_string();
            } else {
                panic!("Field not found!");
            }

            Contact::store_contact(&mut storage, contact);

        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            println!("Contact id:");
            let mut id = String::new();
            io::stdin().read_line(&mut id)
                .expect("Failed to read id");

            let mut contact = Contact::get_contact(&mut storage, id.trim().to_string())
                .expect("Contact not found");

            println!("Contact name: {}(keep blank for skip)", contact.name);
            let mut name = String::new();
            io::stdin().read_line(&mut name)
                .expect("Failed to read name");
            if !name.trim().is_empty() {
                contact.name = name.trim().to_string();
            }

            println!("City: {}(keep blank for skip)", contact.city_location);
            let mut city_location = String::new();
            io::stdin().read_line(&mut city_location)
                .expect("Failed to read city_location");
            if !city_location.trim().is_empty() {
                contact.city_location = city_location.trim().to_string();
            }

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
