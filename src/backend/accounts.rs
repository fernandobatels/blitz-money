///
/// Blitz Money
///
/// Backend of module for manange accounts of user
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use backend::storage::*;
use json::JsonValue;
use chrono::NaiveDate;

#[derive(Default, Clone, Debug)]
pub struct Account {
   pub uuid: String,
   pub bank: String,
   pub name: String,
   pub open_balance: f32,
   pub open_balance_date: Option<NaiveDate>,
   pub currency: String
}

impl Model for Account {

    fn new(row: JsonValue, uuid: String, _storage: &mut Storage) -> Account {

        if row["bank"].is_null() {
            panic!("Bank name not found into a row(id {}) account", uuid);
        }

        if row["name"].is_null() {
            panic!("Name not found into a row(id {}) account", uuid);
        }

        if row["open_balance"].is_null() {
            panic!("Open balance not found into a row(id {}) account", uuid);
        }

        if row["open_balance_date"].is_null() {
            panic!("Open balance date not found into a row(id {}) account", uuid);
        }

        if row["currency"].is_null() {
            panic!("Currency not found into a row(id {}) account", uuid);
        }

        let open_balance_date = Some(NaiveDate::parse_from_str(&row["open_balance_date"].to_string(), "%Y-%m-%d").unwrap());

        Account {
            uuid: uuid,
            bank: row["bank"].to_string(),
            name: row["name"].to_string(),
            open_balance: row["open_balance"].as_f32().unwrap(),
            open_balance_date: open_balance_date,
            currency: row["currency"].to_string()
        }
    }

    fn to_save(self) -> (String, bool, JsonValue) {

        (self.uuid.clone(), self.uuid.is_empty(), object!{
            "bank" => self.bank,
            "name" => self.name,
            "open_balance" => self.open_balance,
            "open_balance_date" => self.open_balance_date.unwrap().format("%Y-%m-%d").to_string(),
            "currency" => self.currency,
        })
    }
}

impl Account {

    // Return the open balance formatted whit currency
    pub fn open_balance_formmated(&self) -> String {
        self.format_value(self.open_balance)
    }

    // Return the value formatted with the currency of account
    pub fn format_value(&self, value: f32) -> String {
        format!("{} {:.2}", self.currency, value)
    }

    // Return a list with all accounts
    pub fn get_accounts(storage: &mut Storage) -> Vec<Account> {

        storage.start_section("accounts".to_string());

        let mut data = storage.get_section_data("accounts".to_string());

        let mut list: Vec<Account> = vec![];

        while let Ok(line) = data.next::<Account>() {
            list.push(line);
        }

        return list;
    }

    // Return the account of id
    pub fn get_account(storage: &mut Storage, uuid: String) -> Result<Account, &'static str> {

        storage.start_section("accounts".to_string());

        let mut data = storage.get_section_data("accounts".to_string());

        if data.find_by_id(uuid) {
            return data.next::<Account>();
        }

        Err("Account not found")
    }

    // Save updates, or create new, account on storage
    pub fn store_account(storage: &mut Storage, account: Account) {

        storage.start_section("accounts".to_string());

        let mut data = storage.get_section_data("accounts".to_string());

        data.save(account);
    }

    // Remvoe account of storage
    pub fn remove_account(storage: &mut Storage, uuid: String) {

        storage.start_section("accounts".to_string());

        let mut data = storage.get_section_data("accounts".to_string());

        data.remove_by_id(uuid);
    }
}
