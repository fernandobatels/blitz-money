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

    fn new(row: JsonValue, uuid: String, _storage: &mut Storage, _can_recursive: bool) -> Account {

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

    // Short version of the uuid
    pub fn id(self) -> String {
        Data::uuid_to_id(self.uuid)
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


#[cfg(test)]
mod tests {

    use super::*;
    use uuid::Uuid;
    use chrono::Local;
    use std::collections::HashMap;
    use std::cell::RefCell;

    fn populate() -> String {

        let path = "/tmp/bmoney-".to_owned() + &Uuid::new_v4().to_string();

        let mut st = Storage { path_str: path.clone(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new()) };

        assert!(st.start_section("accounts".to_string()));

        let mut data = st.get_section_data("accounts".to_string());

        data.save(Account { uuid: "".to_string(), name: "account 1".to_string(), bank: "bank A".to_string(), currency: "R$".to_string(), open_balance: 0.0, open_balance_date: Some(Local::today().naive_local()) });
        data.save(Account { uuid: "".to_string(), name: "account 2".to_string(), bank: "bank B".to_string(), currency: "$".to_string(), open_balance: 100.0, open_balance_date: Some(Local::today().naive_local()) });
        data.save(Account { uuid: "".to_string(), name: "account 3".to_string(), bank: "bank D".to_string(), currency: "$".to_string(), open_balance: -10.0, open_balance_date: Some(Local::today().naive_local()) });
        data.save(Account { uuid: "".to_string(), name: "account 4".to_string(), bank: "bank B".to_string(), currency: "R$".to_string(), open_balance: 35.0, open_balance_date: Some(Local::today().naive_local()) });

        path
    }

    #[test]
    fn get_accounts() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new()) };

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account 4".to_string());
        assert_eq!(accounts[0].bank, "bank B".to_string());
        assert_eq!(accounts[0].currency, "R$".to_string());
        assert_eq!(accounts[0].open_balance, 35.0);
        assert_eq!(accounts[0].open_balance_date, Some(Local::today().naive_local()));

        assert_eq!(accounts[1].name, "account 3".to_string());
        assert_eq!(accounts[1].bank, "bank D".to_string());
        assert_eq!(accounts[1].currency, "$".to_string());
        assert_eq!(accounts[1].open_balance, -10.0);
        assert_eq!(accounts[1].open_balance_date, Some(Local::today().naive_local()));

        assert_eq!(accounts[2].name, "account 2".to_string());
        assert_eq!(accounts[2].bank, "bank B".to_string());
        assert_eq!(accounts[2].currency, "$".to_string());
        assert_eq!(accounts[2].open_balance, 100.0);
        assert_eq!(accounts[2].open_balance_date, Some(Local::today().naive_local()));

        assert_eq!(accounts[3].name, "account 1".to_string());
        assert_eq!(accounts[3].bank, "bank A".to_string());
        assert_eq!(accounts[3].currency, "R$".to_string());
        assert_eq!(accounts[3].open_balance, 0.0);
        assert_eq!(accounts[3].open_balance_date, Some(Local::today().naive_local()));
    }

    #[test]
    fn get_account() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new()) };

        let accounts = Account::get_accounts(&mut st);

        let uuid = accounts[0].uuid.clone();

        let account = Account::get_account(&mut st, uuid);

        assert!(account.is_ok());
        assert_eq!(account.clone().unwrap().name, "account 4".to_string());
        assert_eq!(account.clone().unwrap().bank, "bank B".to_string());
        assert_eq!(account.clone().unwrap().currency, "R$".to_string());
        assert_eq!(account.clone().unwrap().open_balance, 35.0);
        assert_eq!(account.unwrap().open_balance_date, Some(Local::today().naive_local()));
        let accounte = Account::get_account(&mut st, "NOOOO".to_string());

        assert!(accounte.is_err());
    }


    #[test]
    fn store_account() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new()) };

        Account::store_account(&mut st, Account { uuid: "".to_string(), name: "account 5".to_string(), bank: "bank A".to_string(), currency: "R$".to_string(), open_balance: 0.0, open_balance_date: Some(Local::today().naive_local())  });

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account 5".to_string());
        assert_eq!(accounts[0].bank, "bank A".to_string());
        assert_eq!(accounts[0].currency, "R$".to_string());
        assert_eq!(accounts[0].open_balance, 0.0);
        assert_eq!(accounts[0].open_balance_date, Some(Local::today().naive_local()));

        assert_eq!(accounts[1].name, "account 4".to_string());
        assert_eq!(accounts[1].bank, "bank B".to_string());
        assert_eq!(accounts[1].currency, "R$".to_string());
        assert_eq!(accounts[1].open_balance, 35.0);
        assert_eq!(accounts[1].open_balance_date, Some(Local::today().naive_local()));
    }

    #[test]
    fn remove_account() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new()) };

        let accounts = Account::get_accounts(&mut st);

        let uuid = accounts[0].uuid.clone();

        let account = Account::get_account(&mut st, uuid.clone());

        assert!(account.is_ok());

        Account::remove_account(&mut st, uuid.clone());

        let accounte = Account::get_account(&mut st, uuid.clone());

        assert!(accounte.is_err());
    }
}
