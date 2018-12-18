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
use i18n::*;

pub struct Contacts {}

impl Contacts {

    // List of user contacts
    pub fn list(mut storage: Storage, _params: Vec<String>, is_csv: bool) {

        let contacts = Contact::get_contacts(&mut storage);
        let mut table = Output::new_table();

        table.set_titles(row![b->I18n::text("contacts_name"), b->I18n::text("contacts_city_location"), b->"#id"]);

        for contact in contacts {

            table.add_row(row![
                contact.name,
                contact.city_location,
                contact.clone().id()
            ]);
        }

        Output::print_table(table, is_csv);
    }

    // Create new contact
    pub fn add(mut storage: Storage, params: Vec<String>) {

        if params.len() == 2 {
            // Shell mode

            let name = Input::param(I18n::text("contacts_name"), true, params.clone(), 0);
            let city = Input::param(I18n::text("contacts_city_location"), true, params.clone(), 1);

            Contact::store_contact(&mut storage, Contact {
                uuid: "".to_string(),
                city_location: city,
                name: name,
            });
        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let name = Input::read(I18n::text("contacts_name"), true, None);
            let city = Input::read(I18n::text("contacts_city_location"), true, None);

            Contact::store_contact(&mut storage, Contact {
                uuid: "".to_string(),
                city_location: city,
                name: name,
            });
        } else {
            // Help mode
            println!("{}", I18n::text("contacts_how_to_use_add"));
        }
    }

    // Update a existing contact
    pub fn update(mut storage: Storage, params: Vec<String>) {

        if params.len() == 3 {
            // Shell mode

            let mut contact = Contact::get_contact(&mut storage, params[0].trim().to_string())
                .expect(&I18n::text("contacts_not_found"));

            if params[1] == "name" {
                contact.name = Input::param(I18n::text("contacts_name"), true, params.clone(), 2);
            } else if params[1] == "city" {
                contact.city_location = Input::param(I18n::text("contacts_city_location"), true, params.clone(), 2);
            } else {
                panic!(I18n::text("field_not_found"));
            }

            Contact::store_contact(&mut storage, contact);

        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let id = Input::read("#id".to_string(), true, None);

            let mut contact = Contact::get_contact(&mut storage, id)
                .expect(&I18n::text("contacts_not_found"));

            contact.name = Input::read(I18n::text("contacts_name"), true, Some(contact.name));
            contact.city_location = Input::read(I18n::text("contacts_city_location"), true, Some(contact.city_location));

            Contact::store_contact(&mut storage, contact);
        } else {
            // Help mode
            println!("{}", I18n::text("contacts_how_to_use_update"));
        }
    }

    // Remove a existing contact
    pub fn rm(mut storage: Storage, params: Vec<String>) {

        if params.len() == 1 {
            // Shell mode

            Contact::remove_contact(&mut storage, params[0].trim().to_string());
        } else {
            // Help mode
            println!("{}", I18n::text("contacts_how_to_use_rm"));
        }
    }
}
