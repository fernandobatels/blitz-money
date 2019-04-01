///
/// Blitz Money
///
/// Backend of module for manange forecast values of months
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use std::collections::HashMap;
use chrono::NaiveDate;

use backend::storage::*;
use backend::tags::*;
use backend::accounts::*;
use backend::transactions::*;
use backend::contacts::*;
use json::JsonValue;

#[derive(Default, Clone, Debug)]
pub struct Forecast {
   pub uuid: String,
   pub account: Option<Account>,
   pub tag: Option<Tag>,
   pub value: f32
}

impl Model for Forecast {

    fn new(row: JsonValue, uuid: String, storage: &mut Storage, _can_recursive: bool) -> Forecast {

        if row["tag"].is_null() {
            panic!("Tag not found into a row(id {}) forecast", uuid);
        }

        if row["account"].is_null() {
            panic!("Account not found into a row(id {}) forecast", uuid);
        }

        if row["value"].is_null() {
            panic!("Value not found into a row(id {}) forecast", uuid);
        }

        Forecast {
            uuid: uuid,
            tag: Some(Tag::get_tag(storage, row["tag"].to_string()).unwrap()),
            account: Some(Account::get_account(storage, row["account"].to_string()).unwrap()),
            value: row["value"].as_f32().unwrap()
        }
    }

    fn to_save(self) -> (String, bool, JsonValue) {

        let mut ob = object!{
        };

        ob["tag"] = self.tag.unwrap().uuid.into();
        ob["account"] = self.account.unwrap().uuid.into();
        ob["value"] = self.value.into();

        (self.uuid.clone(), self.uuid.is_empty(), ob)
    }
}

impl Forecast {

    // Short version of the uuid
    pub fn id(self) -> String {
        Data::uuid_to_id(self.uuid)
    }

    // Return a list with all forecasts
    pub fn get_forecasts(storage: &mut Storage) -> Vec<Forecast> {

        storage.start_section("forecasts".to_string());

        let mut data = storage.get_section_data("forecasts".to_string());

        let mut list: Vec<Forecast> = vec![];

        while let Ok(line) = data.next::<Forecast>() {
            list.push(line);
        }

        return list;
    }

    // Return the forecast of id
    pub fn get_forecast(storage: &mut Storage, uuid: String) -> Result<Forecast, &'static str> {

        storage.start_section("forecasts".to_string());

        let mut data = storage.get_section_data("forecasts".to_string());

        if data.find_by_id(uuid) {
            return data.next::<Forecast>();
        }

        Err("Forecast not found")
    }

    // Save updates, or create new, forecast on storage
    pub fn store_forecast(storage: &mut Storage, forecast: Forecast) {

        storage.start_section("forecasts".to_string());

        let mut data = storage.get_section_data("forecasts".to_string());

        data.save(forecast);
    }

    // Remvoe forecast of storage
    pub fn remove_forecast(storage: &mut Storage, uuid: String) {

        storage.start_section("forecasts".to_string());

        let mut data = storage.get_section_data("forecasts".to_string());

        data.remove_by_id(uuid);
    }

    // Make transactions for remaining values of tags
    pub fn remaining_transactions(storage: &mut Storage, transactions: Vec<Transaction>, end_date: NaiveDate) -> Vec<Transaction> {

        let mut tags: HashMap<String, f32> = HashMap::new();
        let mut remaining: Vec<Transaction> = vec![];


        for tr in transactions {
            for tag in tr.tags {
                let old_value = tags.entry(tag.uuid).or_insert(0.0);

                *old_value += tr.value;
            }
        }

        for fore in Forecast::get_forecasts(storage) {

            let attain = match tags.get(&fore.clone().tag.unwrap().uuid) {
                Some(at) => at,
                _ => &0.0
            };

            let mut remain = fore.value - attain;

            if (fore.value < 0.0 && remain > 0.0) || (fore.value >= 0.0 && remain < 0.0) {
                remain = 0.0;
            }

            let percent = (attain / fore.value) * 100.0;

            let account = fore.account.clone().unwrap().clone();
            let contact = Contact { uuid: "".to_string(), name: "Remaining".to_string(), city_location: "Remaining".to_string() };

            let description = format!("{}: {} of {} ({:.0}%)", fore.clone().tag.unwrap().name, account.format_value(*attain), account.format_value(fore.value), percent);

            remaining.push(Transaction {
                uuid: "00000000-0000-0000-0000-000000000000".to_string(),
                description: description,
                value: remain,
                account: Some(account),
                contact: Some(contact),
                deadline: Some(end_date),
                ..Default::default()
            });
        }

        remaining
    }
}

/*
#[cfg(test)]
mod tests {

    use super::*;
    use uuid::Uuid;
    use std::collections::HashMap;
    use std::cell::RefCell;

    fn populate() -> String {

        let path = "/tmp/bmoney-".to_owned() + &Uuid::new_v4().to_string();

        let mut st = Storage { path_str: path.clone(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new()) };

        assert!(st.start_section("forecasts".to_string()));

        let mut data = st.get_section_data("forecasts".to_string());

        data.save(Forecast {
            uuid: "".to_string(),
            name: "name A".to_string(),
            value: 1000,
        });
        data.save(Forecast {
            uuid: "".to_string(),
            name: "name B".to_string(),
            value: 200,
        });
        data.save(Forecast {
            uuid: "".to_string(),
            name: "name D".to_string(),
            value: 50,
        });
        data.save(Forecast {
            uuid: "".to_string(),
            name: "name B".to_string(),
            value: 300,
        });

        path
    }

    #[test]
    fn get_forecasts() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new()) };

        let forecasts = Forecast::get_forecasts(&mut st);

        assert_eq!(forecasts[0].description, "forecast 4".to_string());
        assert_eq!(forecasts[0].name, "name B".to_string());
        assert!(!forecasts[0].contact.is_some());
        assert_eq!(forecasts[0].tags.len(), 0);

        assert_eq!(forecasts[3].description, "forecast 1".to_string());
        assert_eq!(forecasts[3].name, "name A".to_string());
        assert!(forecasts[3].contact.is_some());
        assert_eq!(forecasts[3].tags.len(), 0);
    }

    #[test]
    fn apply_forecasts() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new()) };

        let mut tr = Transaction {
            description: "more name b and other".to_string(),
            value: 150.0,
            ..Default::default()
        };

        assert!(Forecast::apply_forecasts(&mut st, &mut tr));

        assert_eq!(tr.description, "forecast 4".to_string());

        let mut tr2 = Transaction {
            description: "more name not found and other".to_string(),
            value: 10.0,
            ..Default::default()
        };

        assert!(!Forecast::apply_forecasts(&mut st, &mut tr2));
    }
}*/
