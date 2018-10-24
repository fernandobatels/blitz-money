///
/// Blitz Money
///
/// Backend of module for manange movimentations
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use backend::storage::*;
use backend::accounts::*;
use backend::contacts::*;
use chrono::{Local, DateTime, NaiveDate};
use json::JsonValue;

#[derive(Clone, Debug)]
pub struct Movimentation {
   pub uuid: String,
   pub account: Option<Account>,
   pub contact: Option<Contact>,
   pub description: String,
   pub value: f32,
   pub deadline: Option<NaiveDate>,
   pub paid_in: Option<NaiveDate>,
   pub created_at: Option<DateTime<Local>>,
   pub updated_at: Option<DateTime<Local>>, // Last update
   pub transaction: Option<Box<Movimentation>>
}

impl Default for Movimentation {

    // Default values, duh
    fn default() -> Movimentation {
        Movimentation {
            uuid: "".to_string(),
            description: "".to_string(),
            value: 0.0,
            account: None,
            contact: None,
            deadline: None,
            paid_in: None,
            created_at: Some(Local::now()),
            updated_at: None,
            transaction: None
        }
    }
}

pub struct Total {
    pub label: String,
    pub value: f32,
}

impl Model for Movimentation {

    fn new(row: JsonValue, uuid: String, storage: &mut Storage, can_recursive: bool) -> Movimentation {

        if row["description"].is_null() {
            panic!("Description not found into a row(id {}) movimentation", uuid);
        }

        if row["value"].is_null() {
            panic!("Value not found into a row(id {}) movimentation", uuid);
        }

        if row["deadline"].is_null() {
            panic!("Deadline not found into a row(id {}) movimentation", uuid);
        }

        if row["contact"].is_null() {
            panic!("Contact not found into a row(id {}) movimentation", uuid);
        }

        if row["account"].is_null() {
            panic!("Account not found into a row(id {}) movimentation", uuid);
        }

        if row["created_at"].is_null() {
            panic!("Created at not found into a row(id {}) movimentation", uuid);
        }

        let mut mov = Movimentation {
            uuid: uuid.clone(),
            description: row["description"].to_string(),
            value: row["value"].as_f32().unwrap(),
            ..Default::default()
        };

        mov.account = Some(Account::get_account(storage, row["account"].to_string()).unwrap());
        mov.contact = Some(Contact::get_contact(storage, row["contact"].to_string()).unwrap());

        mov.created_at = Some(row["created_at"].to_string().parse::<DateTime<Local>>().unwrap());
        mov.deadline = Some(NaiveDate::parse_from_str(&row["deadline"].to_string(), "%Y-%m-%d").unwrap());

        if !row["paid_in"].is_empty() {
            mov.paid_in = Some(NaiveDate::parse_from_str(&row["paid_in"].to_string(), "%Y-%m-%d").unwrap());
        }

        if !row["updated_at"].is_empty() {
            mov.updated_at = Some(row["updated_at"].to_string().parse::<DateTime<Local>>().unwrap());
        }

        if !row["transaction"].is_empty() && can_recursive {

            let mut data = storage.get_section_data("movimentations".to_string());

            if data.find_by_id(row["transaction"].to_string()) {

                // This instruct the new() method of the othor
                // movimentation for dont't run more recursive operations
                data.can_recursive = false;

                let mut other = data.next::<Movimentation>()
                    .expect("Couldn't parse the other transaction");

                // Update the current movimentation with link to
                // movimentation of other account
                mov.transaction = Some(Box::new(other));

                // And link the movimentation of the oter account
                // with this
                other.transaction = Some(Box::new(mov.clone()));
            } else {
                panic!("Couldn't find the movimentation {} need by {}", row["transaction"], uuid);
            }
        }


        mov
    }

    fn to_save(self) -> (String, bool, JsonValue) {

        let mut ob = object!{
            "account" => self.account.unwrap().uuid,
            "contact" => self.contact.unwrap().uuid,
            "description" => self.description,
            "value" => self.value,
            "deadline" => self.deadline.unwrap().format("%Y-%m-%d").to_string(),
            "created_at" => self.created_at.unwrap().to_rfc3339().to_string(),
        };

        if self.paid_in.is_some() {
            ob["paid_in"] = self.paid_in.unwrap().format("%Y-%m-%d").to_string().into();
        }

        if !self.uuid.is_empty() {
            ob["updated_at"] = Local::now().to_rfc3339().to_string().into();
        }

        (self.uuid.clone(), self.uuid.is_empty(), ob)
    }
}

impl Movimentation {

    // Return the value formatted whit currency of account
    pub fn value_formmated(&self) -> String {
        self.account.clone().unwrap().format_value(self.value)
    }

    // Return the paid in formatted
    pub fn paid_in_formmated(&self) -> String {
        if self.paid_in.is_none() {
            return "(payable)".to_string();
        }
        self.paid_in.unwrap().to_string().clone()
    }

    // Return a list with all movimentations
    // and total
    pub fn get_movimentations(storage: &mut Storage, account: Account, from: NaiveDate, to: NaiveDate) -> (Vec<Movimentation>, Vec<Total>) {

        storage.start_section("movimentations".to_string());

        let mut data = storage.get_section_data("movimentations".to_string());
        let mut list: Vec<Movimentation> = vec![];
        let mut totals: Vec<Total> = vec![];

        totals.push(Total { label: "Expenses(payable)".to_string(), value: 0.0 });
        totals.push(Total { label: "Incomes(to receive)".to_string(), value: 0.0 });
        totals.push(Total { label: "Expenses".to_string(), value: 0.0 });
        totals.push(Total { label: "Incomes".to_string(), value: 0.0 });
        totals.push(Total { label: "Previous balance".to_string(), value: account.open_balance });
        totals.push(Total { label: "Current balance".to_string(), value: account.open_balance });

        while let Ok(line) = data.next::<Movimentation>() {
            if account.uuid == line.account.clone().unwrap().uuid {

                // Period filter
                if line.deadline.unwrap() < from || line.deadline.unwrap() > to {

                    if line.deadline.unwrap() < from {
                        totals[4].value += line.value;
                    }

                    continue;
                }

                // Totals
                if line.paid_in.is_some() {

                    totals[5].value += line.value;

                    if line.value >= 0.0 {
                        totals[3].value += line.value;
                    } else {
                        totals[2].value += line.value;
                    }
                } else {

                    if line.value >= 0.0 {
                        totals[1].value += line.value;
                    } else {
                        totals[0].value += line.value;
                    }
                }

                list.push(line);
            }
        }

        list.sort_by( | a, b | a.deadline.unwrap().cmp(&b.deadline.unwrap()) );

        return (list, totals);
    }

    // Return the movimentation of id
    pub fn get_movimentation(storage: &mut Storage, uuid: String) -> Result<Movimentation, &'static str> {

        storage.start_section("movimentations".to_string());

        let mut data = storage.get_section_data("movimentations".to_string());

        if data.find_by_id(uuid) {
            return data.next::<Movimentation>();
        }

        Err("Movimentation not found")
    }

    // Save updates, or create new, movimentation on storage
    pub fn store_movimentation(storage: &mut Storage, movimentation: Movimentation) {

        storage.start_section("movimentations".to_string());

        let mut data = storage.get_section_data("movimentations".to_string());

        data.save(movimentation);
    }

    // Remvoe movimentation of storage
    pub fn remove_movimentation(storage: &mut Storage, uuid: String) {

        storage.start_section("movimentations".to_string());

        let mut data = storage.get_section_data("movimentations".to_string());

        data.remove_by_id(uuid);
    }
}
