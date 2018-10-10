///
/// Blitz Money
///
/// Backend of module for manange accounts of user
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use backend::storage::*;
use json::JsonValue;

#[derive(Default, Clone, Debug)]
pub struct Account {
   pub uuid: String,
   pub bank: String,
   pub name: String,
}

impl Model for Account {

    fn new(row: JsonValue, uuid: String) -> Account {

        if row["bank"].is_null() {
            panic!("Bank name not found into a row(id {}) account", row["id"]);
        }

        if row["name"].is_null() {
            panic!("Name not found into a row(id {}) account", row["id"]);
        }

        Account{ uuid: uuid, bank: row["bank"].to_string(), name: row["name"].to_string() }
    }

    fn to_save(self) -> (String, bool, JsonValue) {

        (self.uuid.clone(), self.uuid.is_empty(), object!{
            "bank" => self.bank,
            "name" => self.name,
        })
    }
}

impl Account {

    pub fn get_accounts(mut storage: Storage) -> Vec<Account> {

        storage.start_section("accounts".to_string());

        let mut data = storage.get_section_data("accounts".to_string());

        let mut list: Vec<Account> = vec![];

        while let Ok(line) = data.next::<Account>() {
            list.push(line);
        }

        return list;
    }
}
