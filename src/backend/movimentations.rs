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
use backend::tags::*;
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
   pub transfer: Option<Box<Movimentation>>,
   pub tags: Vec<Tag>,
   pub observations: String,
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
            transfer: None,
            tags: vec!(),
            observations: "".to_string()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Total {
    pub label: String,
    pub value: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StatusFilter {
    FORPAY,
    PAID,
    ALL
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

        if row["contact"].is_null() && row["transfer"].is_null() {
            // We dont need contact if is a transfer
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

        if !row["contact"].is_empty() {
            mov.contact = Some(Contact::get_contact(storage, row["contact"].to_string()).unwrap());
        } else {
            mov.contact = None;
        }

        mov.created_at = Some(row["created_at"].to_string().parse::<DateTime<Local>>().unwrap());
        mov.deadline = Some(NaiveDate::parse_from_str(&row["deadline"].to_string(), "%Y-%m-%d").unwrap());

        if !row["paid_in"].is_empty() {
            mov.paid_in = Some(NaiveDate::parse_from_str(&row["paid_in"].to_string(), "%Y-%m-%d").unwrap());
        }

        if !row["updated_at"].is_empty() {
            mov.updated_at = Some(row["updated_at"].to_string().parse::<DateTime<Local>>().unwrap());
        }

        if !row["transfer"].is_empty() && can_recursive {

            let mut data = storage.get_section_data("movimentations".to_string());

            if data.find_by_id(row["transfer"].to_string()) {

                // This instruct the new() method of the other
                // movimentation for dont't run more recursive operations
                data.can_recursive = false;

                let mut other = data.next::<Movimentation>()
                    .expect("Couldn't parse the other transfer");

                // Update the current movimentation with link to
                // movimentation of other account
                mov.transfer = Some(Box::new(other));

                // And link the movimentation of the oter account
                // with this
                other.transfer = Some(Box::new(mov.clone()));
            } else {
                panic!("Couldn't find the movimentation {} need by {}", row["transfer"], uuid);
            }
        }

        if !row["tags"].is_empty() {
            for stag in row["tags"].members() {
                if let Ok(tag) = Tag::get_tag(storage, stag.to_string()) {
                    mov.tags.push(tag);
                }
            }
        }

        if !row["observations"].is_empty() {
            mov.observations = row["observations"].to_string();
        }

        mov
    }

    fn to_save(self) -> (String, bool, JsonValue) {

        let mut ob = object!{
            "account" => self.account.unwrap().uuid,
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

        if self.contact.is_some() {
            ob["contact"] = self.contact.unwrap().uuid.into();
        } else if self.transfer.is_none() {
            panic!("Contact or transfer must be present!");
        }

        if self.transfer.is_some() {
            ob["transfer"] = self.transfer.unwrap().uuid.into();
        }

        if self.tags.len() > 0 {
            let tags: Vec<String> = self.tags
                .iter()
                .map(|tag| tag.uuid.clone())
                .collect();

            ob["tags"] = tags.into();
        }

        if !self.observations.is_empty() {
            ob["observations"] = self.observations.into();
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
    pub fn get_movimentations(storage: &mut Storage, account: Account, from: NaiveDate, to: NaiveDate, filter_status: StatusFilter, filter_uuid: Option<String>, filter_tag: Option<String>) -> (Vec<Movimentation>, Vec<Total>) {

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

                    if line.deadline.unwrap() < from && line.paid_in.is_some() {
                        totals[4].value += line.value;
                    }

                    continue;
                }

                let mut filter_status_ok: bool;

                // Totals
                if line.paid_in.is_some() {

                    totals[5].value += line.value;

                    if line.value >= 0.0 {
                        totals[3].value += line.value;
                    } else {
                        totals[2].value += line.value;
                    }

                    filter_status_ok = StatusFilter::PAID == filter_status || StatusFilter::ALL == filter_status;
                } else {

                    if line.value >= 0.0 {
                        totals[1].value += line.value;
                    } else {
                        totals[0].value += line.value;
                    }

                    filter_status_ok = StatusFilter::FORPAY == filter_status || StatusFilter::ALL == filter_status;
                }

                let mut filter_uuid_ok = true;
                if let Some(fuuid) = filter_uuid.clone() {
                    filter_status_ok = fuuid == line.uuid;
                }

                let mut filter_tag_ok = true;
                if let Some(ftag) = filter_tag.clone() {
                    filter_tag_ok = line.tags.iter().any(|tag| ftag == tag.uuid);
                }

                if filter_status_ok && filter_uuid_ok && filter_tag_ok {
                    list.push(line);
                }

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

        if movimentation.transfer.is_some() {
            panic!("You must be use the Movimentation#store_transfer for transfers!");
        }

        storage.start_section("movimentations".to_string());

        let mut data = storage.get_section_data("movimentations".to_string());

        data.save(movimentation);
    }

    // Save updates, or create new, transfers movimentations
    pub fn store_transfer(storage: &mut Storage, movimentation: &mut Movimentation, other: &mut Movimentation) {

        storage.start_section("movimentations".to_string());

        let mut data = storage.get_section_data("movimentations".to_string());

        // The absolute value must be the same, duh
        if movimentation.value.abs() != other.value.abs() {
            other.value = movimentation.value;
        }

        // And the deadline
        if movimentation.deadline != other.deadline {
            other.deadline = movimentation.deadline;
        }

        // And the paid in
        if movimentation.paid_in != other.paid_in {
            other.paid_in = movimentation.paid_in;
        }

        // If value is the same we must invert the
        // value of second movimentation
        if movimentation.value == other.value {
            if movimentation.value >= 0.0 {
                other.value = 0.0 - movimentation.value;
            } else {
                other.value = movimentation.value.abs();
            }
        }

        // If is a insert
        if movimentation.uuid.is_empty() || other.uuid.is_empty() {

            // We save the first movimentation to get his uuid
            // and put on the second
            movimentation.uuid = data.save(movimentation.to_owned());
            other.transfer = Some(Box::new(movimentation.clone()));

            // Now, we save de second movimentation to get his
            // uuid and put on the first. On this point the second
            // has the first uuid
            other.uuid = data.save(other.to_owned());
            movimentation.transfer = Some(Box::new(other.clone()));

            // And finaly, we store the first for save the
            // uuid of second
            data.save(movimentation.to_owned());
        } else {
            // Update

            other.transfer = Some(Box::new(movimentation.clone()));
            movimentation.transfer = Some(Box::new(other.clone()));

            data.save(movimentation.to_owned());
            data.save(other.to_owned());
        }
    }

    // Remvoe movimentation of storage
    pub fn remove_movimentation(storage: &mut Storage, uuid: String) {

        storage.start_section("movimentations".to_string());

        let mut data = storage.get_section_data("movimentations".to_string());

        if data.find_by_id(uuid.clone()) {
            let mov = data.next::<Movimentation>()
                .expect("Clound't parse the movimentation");

            if mov.transfer.is_some() {
                data.remove_by_id(mov.transfer.unwrap().uuid);
            }
        }

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

        Account::store_account(&mut st, Account { uuid: "".to_string(), name: "account AA".to_string(), bank: "bank A".to_string(), currency: "R$".to_string(), open_balance: 0.0, open_balance_date: Some(Local::today().naive_local()) });
        Account::store_account(&mut st, Account { uuid: "".to_string(), name: "account BB".to_string(), bank: "bank B".to_string(), currency: "R$".to_string(), open_balance: 35.0, open_balance_date: Some(Local::today().naive_local()) });

        let accounts = Account::get_accounts(&mut st);

        Contact::store_contact(&mut st, Contact { uuid: "".to_string(), name: "contact 1".to_string(), city_location: "city A".to_string() });
        Contact::store_contact(&mut st, Contact { uuid: "".to_string(), name: "contact 2".to_string(), city_location: "city B".to_string() });

        let contacts = Contact::get_contacts(&mut st);

        assert!(st.start_section("movimentations".to_string()));

        let mut data = st.get_section_data("movimentations".to_string());

        data.save(Movimentation {
            description: "movimentation 1".to_string(),
            value: 10.00,
            account: Some(accounts[0].clone()),
            contact: Some(contacts[0].clone()),
            deadline: Some(NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap()),
            ..Default::default()
        });
        data.save(Movimentation {
            description: "movimentation 2".to_string(),
            value: -125.53,
            account: Some(accounts[0].clone()),
            contact: Some(contacts[1].clone()),
            deadline: Some(NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap()),
            ..Default::default()
        });
        data.save(Movimentation {
            description: "movimentation 3".to_string(),
            value: 25.58,
            account: Some(accounts[1].clone()),
            contact: Some(contacts[1].clone()),
            deadline: Some(NaiveDate::parse_from_str("2018-10-15", "%Y-%m-%d").unwrap()),
            ..Default::default()
        });
        data.save(Movimentation {
            description: "movimentation 4".to_string(),
            value: 159.02,
            account: Some(accounts[0].clone()),
            contact: Some(contacts[0].clone()),
            deadline: Some(NaiveDate::parse_from_str("2018-08-23", "%Y-%m-%d").unwrap()),
            ..Default::default()
        });

        path
    }

    #[test]
    fn get_movimentations() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new() };

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account BB".to_string());

        let (movimentations, _total) = Movimentation::get_movimentations(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, None);

        assert_eq!(movimentations.len(), 2);
        assert_eq!(movimentations[0].description, "movimentation 2".to_string());
        assert_eq!(movimentations[1].description, "movimentation 1".to_string());
    }

    #[test]
    fn get_movimentations_totals() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new() };

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account BB".to_string());
        assert_eq!(accounts[1].name, "account AA".to_string());

        let (movimentations_a, totals_a) = Movimentation::get_movimentations(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, None);

        assert_eq!(movimentations_a.len(), 2);

        assert_eq!(totals_a.len(), 6);
        assert_eq!(totals_a[0].label, "Expenses(payable)".to_string());
        assert_eq!(totals_a[0].value, -125.53);
        assert_eq!(totals_a[1].label, "Incomes(to receive)".to_string());
        assert_eq!(totals_a[1].value, 10.0);
        assert_eq!(totals_a[2].label, "Expenses".to_string());
        assert_eq!(totals_a[2].value, 0.0);
        assert_eq!(totals_a[3].label, "Incomes".to_string());
        assert_eq!(totals_a[3].value, 0.0);
        assert_eq!(totals_a[4].label, "Previous balance".to_string());
        assert_eq!(totals_a[4].value, 35.0);
        assert_eq!(totals_a[5].label, "Current balance".to_string());
        assert_eq!(totals_a[5].value, 35.0);

        let mut paid = movimentations_a[0].clone();
        paid.paid_in = Some(NaiveDate::parse_from_str("2018-10-25", "%Y-%m-%d").unwrap());
        Movimentation::store_movimentation(&mut st, paid);

        let (movimentations_b, totals_b) = Movimentation::get_movimentations(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, None);

        assert_eq!(movimentations_b.len(), 2);

        assert_eq!(totals_b.len(), 6);
        assert_eq!(totals_b[0].label, "Expenses(payable)".to_string());
        assert_eq!(totals_b[0].value, 0.0);
        assert_eq!(totals_b[1].label, "Incomes(to receive)".to_string());
        assert_eq!(totals_b[1].value, 10.0);
        assert_eq!(totals_b[2].label, "Expenses".to_string());
        assert_eq!(totals_b[2].value, -125.53);
        assert_eq!(totals_b[3].label, "Incomes".to_string());
        assert_eq!(totals_b[3].value, 0.0);
        assert_eq!(totals_b[4].label, "Previous balance".to_string());
        assert_eq!(totals_b[4].value, 35.0);
        assert_eq!(totals_b[5].label, "Current balance".to_string());
        assert_eq!(totals_b[5].value, -90.53);

        let (movimentations_c, totals_c) = Movimentation::get_movimentations(&mut st, accounts[1].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, None);

        assert_eq!(movimentations_c.len(), 1);

        assert_eq!(totals_c.len(), 6);
        assert_eq!(totals_c[0].label, "Expenses(payable)".to_string());
        assert_eq!(totals_c[0].value, 0.0);
        assert_eq!(totals_c[1].label, "Incomes(to receive)".to_string());
        assert_eq!(totals_c[1].value, 25.580002);
        assert_eq!(totals_c[2].label, "Expenses".to_string());
        assert_eq!(totals_c[2].value, 0.0);
        assert_eq!(totals_c[3].label, "Incomes".to_string());
        assert_eq!(totals_c[3].value, 0.0);
        assert_eq!(totals_c[4].label, "Previous balance".to_string());
        assert_eq!(totals_c[4].value, 0.0);
        assert_eq!(totals_c[5].label, "Current balance".to_string());
        assert_eq!(totals_c[5].value, 0.0);
    }

    #[test]
    fn transfers() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new() };

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account BB".to_string());
        assert_eq!(accounts[1].name, "account AA".to_string());

        let contacts = Contact::get_contacts(&mut st);

        assert_eq!(contacts[0].name, "contact 2".to_string());

        let mut from = Movimentation {
            description: "movimentation from".to_string(),
            value: 20.00,
            account: Some(accounts[0].clone()),
            contact: Some(contacts[0].clone()),
            deadline: Some(NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap()),
            ..Default::default()
        };

        let mut to = from.clone();
        to.account = Some(accounts[1].clone());
        to.description = "movimentation to".to_string();

        Movimentation::store_transfer(&mut st, &mut from, &mut to);

        let (movimentations_a, totals_a) = Movimentation::get_movimentations(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, None);

        assert_eq!(movimentations_a.len(), 3);

        assert_eq!(totals_a.len(), 6);
        assert_eq!(totals_a[1].label, "Incomes(to receive)".to_string());
        assert_eq!(totals_a[1].value, 30.0);

        let (movimentations_b, totals_b) = Movimentation::get_movimentations(&mut st, accounts[1].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, None);

        assert_eq!(movimentations_b.len(), 2);

        assert_eq!(totals_b.len(), 6);
        assert_eq!(totals_b[0].label, "Expenses(payable)".to_string());
        assert_eq!(totals_b[0].value, -20.0);

        // Paying the transfer
        let mut paid = movimentations_a[0].clone();
        assert_eq!(paid.description, "movimentation from".to_string());
        paid.paid_in = Some(NaiveDate::parse_from_str("2018-10-19", "%Y-%m-%d").unwrap());
        Movimentation::store_transfer(&mut st, &mut paid.clone(), &mut paid.transfer.unwrap());

        let (movimentations_c, totals_c) = Movimentation::get_movimentations(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, None);

        assert_eq!(movimentations_c.len(), 3);

        assert_eq!(totals_c.len(), 6);
        assert_eq!(totals_c[1].label, "Incomes(to receive)".to_string());
        assert_eq!(totals_c[1].value, 10.0);

        let (movimentations_d, totals_d) = Movimentation::get_movimentations(&mut st, accounts[1].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, None);

        assert_eq!(movimentations_d.len(), 2);

        assert_eq!(totals_d.len(), 6);
        assert_eq!(totals_d[0].label, "Expenses(payable)".to_string());
        assert_eq!(totals_d[0].value, 0.0);
    }

    #[test]
    fn get_movimentations_status() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new() };

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account BB".to_string());

        let (movimentations_tmp, _totals) = Movimentation::get_movimentations(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, None);

        assert_eq!(movimentations_tmp.len(), 2);

        let mut paid = movimentations_tmp[0].clone();
        paid.paid_in = Some(NaiveDate::parse_from_str("2018-10-25", "%Y-%m-%d").unwrap());
        Movimentation::store_movimentation(&mut st, paid);

        let (movimentations_a, totals_a) = Movimentation::get_movimentations(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, None);

        assert_eq!(movimentations_a.len(), 2);

        assert_eq!(totals_a.len(), 6);
        assert_eq!(totals_a[0].label, "Expenses(payable)".to_string());
        assert_eq!(totals_a[0].value, 0.0);
        assert_eq!(totals_a[1].label, "Incomes(to receive)".to_string());
        assert_eq!(totals_a[1].value, 10.0);
        assert_eq!(totals_a[2].label, "Expenses".to_string());
        assert_eq!(totals_a[2].value, -125.53);
        assert_eq!(totals_a[3].label, "Incomes".to_string());
        assert_eq!(totals_a[3].value, 0.0);
        assert_eq!(totals_a[4].label, "Previous balance".to_string());
        assert_eq!(totals_a[4].value, 35.0);
        assert_eq!(totals_a[5].label, "Current balance".to_string());
        assert_eq!(totals_a[5].value, -90.53);

        let (movimentations_b, totals_b) = Movimentation::get_movimentations(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::PAID, None, None);

        assert_eq!(movimentations_b.len(), 1);

        assert_eq!(totals_b.len(), 6);
        assert_eq!(totals_b[0].label, "Expenses(payable)".to_string());
        assert_eq!(totals_b[0].value, 0.0);
        assert_eq!(totals_b[1].label, "Incomes(to receive)".to_string());
        assert_eq!(totals_b[1].value, 10.0);
        assert_eq!(totals_b[2].label, "Expenses".to_string());
        assert_eq!(totals_b[2].value, -125.53);
        assert_eq!(totals_b[3].label, "Incomes".to_string());
        assert_eq!(totals_b[3].value, 0.0);
        assert_eq!(totals_b[4].label, "Previous balance".to_string());
        assert_eq!(totals_b[4].value, 35.0);
        assert_eq!(totals_b[5].label, "Current balance".to_string());
        assert_eq!(totals_b[5].value, -90.53);

        let (movimentations_c, totals_c) = Movimentation::get_movimentations(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::FORPAY, None, None);

        assert_eq!(movimentations_c.len(), 1);

        assert_eq!(totals_c.len(), 6);
        assert_eq!(totals_c[0].label, "Expenses(payable)".to_string());
        assert_eq!(totals_c[0].value, 0.0);
        assert_eq!(totals_c[1].label, "Incomes(to receive)".to_string());
        assert_eq!(totals_c[1].value, 10.0);
        assert_eq!(totals_c[2].label, "Expenses".to_string());
        assert_eq!(totals_c[2].value, -125.53);
        assert_eq!(totals_c[3].label, "Incomes".to_string());
        assert_eq!(totals_c[3].value, 0.0);
        assert_eq!(totals_c[4].label, "Previous balance".to_string());
        assert_eq!(totals_c[4].value, 35.0);
        assert_eq!(totals_c[5].label, "Current balance".to_string());
        assert_eq!(totals_c[5].value, -90.53);
    }

    #[test]
    fn get_movimentation() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new() };

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account BB".to_string());

        let (movimentations, _total) = Movimentation::get_movimentations(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, None);

        let uuid = movimentations[0].uuid.clone();

        let movimentation = Movimentation::get_movimentation(&mut st, uuid);

        assert!(movimentation.is_ok());
        assert_eq!(movimentation.unwrap().description, "movimentation 2".to_string());

        let movimentatione = Movimentation::get_movimentation(&mut st, "NOOOO".to_string());

        assert!(movimentatione.is_err());
    }

    #[test]
    fn store_movimentation() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new() };

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account BB".to_string());

        let contacts = Contact::get_contacts(&mut st);

        assert_eq!(contacts[0].name, "contact 2".to_string());

        Movimentation::store_movimentation(&mut st, Movimentation {
            description: "movimentation 5".to_string(),
            value: 20.00,
            account: Some(accounts[0].clone()),
            contact: Some(contacts[0].clone()),
            deadline: Some(NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap()),
            ..Default::default()
        });

        let (movimentations, _total) = Movimentation::get_movimentations(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, None);

        assert_eq!(movimentations[0].description, "movimentation 5".to_string());
        assert_eq!(movimentations[1].description, "movimentation 2".to_string());
        assert_eq!(movimentations[2].description, "movimentation 1".to_string());
    }

    #[test]
    fn remove_movimentation() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new() };

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account BB".to_string());

        let (movimentations, _total) = Movimentation::get_movimentations(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, None);

        let uuid = movimentations[0].uuid.clone();

        let movimentation = Movimentation::get_movimentation(&mut st, uuid.clone());

        assert!(movimentation.is_ok());

        Movimentation::remove_movimentation(&mut st, uuid.clone());

        let movimentatione = Movimentation::get_movimentation(&mut st, uuid.clone());

        assert!(movimentatione.is_err());
    }
}
