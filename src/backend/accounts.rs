///
/// Blitz Money
///
/// Module for manange accounts of user
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use qtbindingsinterface::AccountsList;
use qtbindingsinterface::AccountsEmitter;
use qtbindingsinterface::AccountsTrait;
use backend::storage::{LOCKED_STORAGE, Model};
use json::JsonValue;

#[derive(Default, Clone)]
struct Account {
    uuid: String,
    bank: String,
    name: String,
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

pub struct Accounts {
    emit: AccountsEmitter,
    #[allow(dead_code)] // Used on qml
    model: AccountsList,
    list: Vec<Account>,
}

impl AccountsTrait for Accounts {

    fn new(emit: AccountsEmitter, model: AccountsList) -> Accounts {

        let mut storage = LOCKED_STORAGE.lock().unwrap();
        storage.start_section("accounts".to_string());

        let mut data = storage.get_section_data("accounts".to_string());

        let mut ac = Accounts {
            emit: emit,
            model: model,
            list: [].to_vec()
        };

        while let Ok(line) = data.next::<Account>() {
            ac.list.push(line);
        }

        ac
    }

    fn emit(&self) -> &AccountsEmitter {
        &self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }

    fn bank(&self, index: usize) -> &str {
        &self.list[index].bank
    }

    fn set_bank(&mut self, index: usize, v: String) -> bool {
        self.list[index].bank = v;
        true
    }

    fn uuid(&self, index: usize) -> &str {
        &self.list[index].uuid
    }

    fn set_uuid(&mut self, index: usize, v: String) -> bool {
        self.list[index].uuid = v;
        true
    }

    fn name(&self, index: usize) -> &str {
        &self.list[index].name
    }

    fn set_name(&mut self, index: usize, v: String) -> bool {
        self.list[index].name = v;
        true
    }
}
