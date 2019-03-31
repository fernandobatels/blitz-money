///
/// Blitz Money
///
/// Backend of module for manange transactions
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use backend::storage::*;
use backend::accounts::*;
use backend::contacts::*;
use backend::tags::*;
use i18n::*;
use chrono::{Local, DateTime, NaiveDate};
use json::JsonValue;

#[derive(Clone, Debug)]
pub struct Transaction {
   pub uuid: String,
   pub account: Option<Account>,
   pub contact: Option<Contact>,
   pub description: String,
   pub value: f32,
   // When this transaction will be paid
   pub deadline: Option<NaiveDate>,
   pub paid_in: Option<NaiveDate>,
   pub created_at: Option<DateTime<Local>>,
   // Last update
   pub updated_at: Option<DateTime<Local>>,
   // When this transaction is a transfer. If has value on this field the field 'contact' will be empty
   pub transfer: Option<Box<Transaction>>,
   pub tags: Vec<Tag>,
   pub observations: String,
   // OFX references
   pub ofx_memo: String,
   pub ofx_fitid: String,
    // Link to the principal transaction, when this is merged into her
   pub merged_in: String,
    // Link to the previuos transaction on payment installments, for example, or
    // used in transactions created repeatedly
   pub previous_repetition: String
}

impl Default for Transaction {

    // Default values, duh
    fn default() -> Transaction {
        Transaction {
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
            observations: "".to_string(),
            ofx_memo: "".to_string(),
            ofx_fitid: "".to_string(),
            merged_in: "".to_string(),
            previous_repetition: "".to_string()
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

impl Model for Transaction {

    fn new(row: JsonValue, uuid: String, storage: &mut Storage, can_recursive: bool) -> Transaction {

        if row["description"].is_null() {
            panic!("Description not found into a row(id {}) transaction", uuid);
        }

        if row["value"].is_null() {
            panic!("Value not found into a row(id {}) transaction", uuid);
        }

        if row["deadline"].is_null() {
            panic!("Deadline not found into a row(id {}) transaction", uuid);
        }

        if row["contact"].is_null() && row["transfer"].is_null() {
            // We dont need contact if is a transfer
            panic!("Contact not found into a row(id {}) transaction", uuid);
        }

        if row["account"].is_null() {
            panic!("Account not found into a row(id {}) transaction", uuid);
        }

        if row["created_at"].is_null() {
            panic!("Created at not found into a row(id {}) transaction", uuid);
        }

        let mut mov = Transaction {
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

            let mut data = storage.get_section_data("transactions".to_string());

            if data.find_by_id(row["transfer"].to_string()) {

                // This instruct the new() method of the other
                // transaction for dont't run more recursive operations
                data.can_recursive = false;

                let mut other = data.next::<Transaction>()
                    .expect("Couldn't parse the other transfer");

                // Update the current transaction with link to
                // transaction of other account
                mov.transfer = Some(Box::new(other));

                // And link the transaction of the oter account
                // with this
                other.transfer = Some(Box::new(mov.clone()));
            } else {
                panic!("Couldn't find the transaction {} need by {}", row["transfer"], uuid);
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

        if !row["ofx_memo"].is_empty() && !row["ofx_fitid"].is_empty() {
            // We only accept, for a valid ofx reference, if have a fitid
            mov.ofx_memo = row["ofx_memo"].to_string();
            mov.ofx_fitid = row["ofx_fitid"].to_string();
        }

        if !row["merged_in"].is_empty() {
            mov.merged_in = row["merged_in"].to_string();
        }

        if !row["previous_repetition"].is_empty() {
            mov.previous_repetition = row["previous_repetition"].to_string();
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

        if !self.ofx_memo.is_empty() && !self.ofx_fitid.is_empty() {
            ob["ofx_memo"] = self.ofx_memo.into();
            ob["ofx_fitid"] = self.ofx_fitid.into();
        }

        if !self.merged_in.is_empty() {
            ob["merged_in"] = self.merged_in.into();
        }

        if !self.previous_repetition.is_empty() {
            ob["previous_repetition"] = self.previous_repetition.into();
        }

        (self.uuid.clone(), self.uuid.is_empty(), ob)
    }
}

impl Transaction {

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

    // Short version of the uuid
    pub fn id(self) -> String {
        Data::uuid_to_id(self.uuid)
    }

    // Return a list with all transactions of account
    pub fn get_transactions_simple(storage: &mut Storage, account: Account) -> Vec<Transaction> {

        storage.start_section("transactions".to_string());

        let mut data = storage.get_section_data("transactions".to_string());
        let mut list: Vec<Transaction> = vec![];

        while let Ok(line) = data.next::<Transaction>() {
            if account.uuid == line.account.clone().unwrap().uuid {
                list.push(line);
            }
        }

        list
    }

    // Return a list with all transactions, except the mergeds, of account and totals, with more filters
    pub fn get_transactions(storage: &mut Storage, account: Account, from: NaiveDate, to: NaiveDate, filter_status: StatusFilter, filter_tag: Option<Tag>, show_mergeds: bool) -> (Vec<Transaction>, Vec<Total>) {

        storage.start_section("transactions".to_string());

        let mut data = storage.get_section_data("transactions".to_string());
        let mut list: Vec<Transaction> = vec![];
        let mut totals: Vec<Total> = vec![];

        let i_tep = 0;
        totals.push(Total { label: I18n::text("transactions_expenses_payable"), value: 0.0 });
        let i_te = 1;
        totals.push(Total { label: I18n::text("transactions_expenses"), value: 0.0 });
        let i_tfo = 2;
        totals.push(Total { label: I18n::text("transactions_transfers_out"), value: 0.0 });
        let i_tit = 3;
        totals.push(Total { label: I18n::text("transactions_incomes_toreceive"), value: 0.0 });
        let i_ti = 4;
        totals.push(Total { label: I18n::text("transactions_incomes"), value: 0.0 });
        let i_tfi = 5;
        totals.push(Total { label: I18n::text("transactions_transfers_in"), value: 0.0 });
        let i_tpb = 6;
        totals.push(Total { label: I18n::text("transactions_previous_balance"), value: account.open_balance });
        let i_tpeb = 7;
        totals.push(Total { label: I18n::text("transactions_previous_expected_balance"), value: account.open_balance });
        let i_teb = 8;
        totals.push(Total { label: I18n::text("transactions_expeected_balance"), value: account.open_balance });
        let i_tcb = 9;
        totals.push(Total { label: I18n::text("transactions_current_balance"), value: account.open_balance });

        while let Ok(line) = data.next::<Transaction>() {
            if account.uuid == line.account.clone().unwrap().uuid && line.merged_in.is_empty() {

                // Filter by status
                if line.paid_in.is_some() && filter_status == StatusFilter::FORPAY {
                    continue;
                } else if line.paid_in.is_none() && filter_status == StatusFilter::PAID {
                    continue;
                }

                // Filter by tag
                if let Some(ftag) = filter_tag.clone() {
                    if !(line.tags.iter().any(|tag| ftag.uuid == tag.uuid)) {
                        continue;
                    }
                }

                // Totals: Previous + Total balance
                if line.paid_in.is_some() {

                    if line.deadline.unwrap() <= to {
                        // We cant sum the future transactions because the user can
                        // make scheduled transactions for nexts years or filter by
                        // previous months
                        totals[i_tcb].value += line.value;
                    }

                    if line.deadline.unwrap() < from {
                        totals[i_tpb].value += line.value;
                    }
                }

                // Totals: Previous expected balance
                if line.deadline.unwrap() < from {
                    totals[i_tpeb].value += line.value;
                }

                // Totals: Expected balance
                if line.deadline.unwrap() <= to {
                    // We cant sum the future transactions because the user can
                    // make scheduled transactions for nexts years
                    totals[i_teb].value += line.value;
                }

                // Period filter
                if line.deadline.unwrap() < from || line.deadline.unwrap() > to {
                    continue;
                }

                if line.paid_in.is_some() {

                    if line.transfer.is_some() {
                        // Totals: Transfer in + Transfers out
                        if line.value >= 0.0 {
                            totals[i_tfi].value += line.value;
                        } else {
                            totals[i_tfo].value += line.value;
                        }
                    } else {
                        // Totals: Expenses + Incomes paids
                        if line.value >= 0.0 {
                            totals[i_ti].value += line.value;
                        } else {
                            totals[i_te].value += line.value;
                        }
                    }
                } else {

                    // Totals: Expenses + Incomes payables
                    if line.value >= 0.0 {
                        totals[i_tit].value += line.value;
                    } else {
                        totals[i_tep].value += line.value;
                    }
                }

                list.push(line);
            } else if account.uuid == line.account.clone().unwrap().uuid && !line.merged_in.is_empty() && show_mergeds && line.deadline.unwrap() >= from && line.deadline.unwrap() <= to {
                // Merged transactions will be not considered in totals
                list.push(line);
            }
        }

        list.sort_by( | a, b | a.deadline.unwrap().cmp(&b.deadline.unwrap()) );

        return (list, totals);
    }

    // Return the transaction of id
    pub fn get_transaction(storage: &mut Storage, uuid: String) -> Result<Transaction, &'static str> {

        storage.start_section("transactions".to_string());

        let mut data = storage.get_section_data("transactions".to_string());

        if data.find_by_id(uuid) {
            return data.next::<Transaction>();
        }

        Err("Transaction not found")
    }

    // Save updates, or create new, transaction on storage
    pub fn store_transaction(storage: &mut Storage, transaction: Transaction) -> String {

        if transaction.transfer.is_some() {
            panic!("You must be use the Transaction#store_transfer for transfers!");
        }

        storage.start_section("transactions".to_string());

        let mut data = storage.get_section_data("transactions".to_string());

        data.save(transaction)
    }

    // Save updates, or create new, transfers transactions
    pub fn store_transfer(storage: &mut Storage, transaction: &mut Transaction, other: &mut Transaction) -> String {

        storage.start_section("transactions".to_string());

        let mut data = storage.get_section_data("transactions".to_string());

        // The absolute value must be the same, duh
        if transaction.value.abs() != other.value.abs() {
            other.value = transaction.value;
        }

        // And the deadline
        if transaction.deadline != other.deadline {
            other.deadline = transaction.deadline;
        }

        // And the paid in
        if transaction.paid_in != other.paid_in {
            other.paid_in = transaction.paid_in;
        }

        // If value is the same we must invert the
        // value of second transaction
        if transaction.value == other.value {
            if transaction.value >= 0.0 {
                other.value = 0.0 - transaction.value;
            } else {
                other.value = transaction.value.abs();
            }
        }

        // If is a insert
        if transaction.uuid.is_empty() || other.uuid.is_empty() {

            // We save the first transaction to get his uuid
            // and put on the second
            transaction.uuid = data.save(transaction.to_owned());
            other.transfer = Some(Box::new(transaction.clone()));

            // Now, we save de second transaction to get his
            // uuid and put on the first. On this point the second
            // has the first uuid
            other.uuid = data.save(other.to_owned());
            transaction.transfer = Some(Box::new(other.clone()));

            // And finaly, we store the first for save the
            // uuid of second
            data.save(transaction.to_owned());
        } else {
            // Update

            other.transfer = Some(Box::new(transaction.clone()));
            transaction.transfer = Some(Box::new(other.clone()));

            data.save(transaction.to_owned());
            data.save(other.to_owned());
        }

        transaction.uuid.clone()
    }

    // Remvoe transaction of storage
    pub fn remove_transaction(storage: &mut Storage, uuid: String) {

        storage.start_section("transactions".to_string());

        let mut data = storage.get_section_data("transactions".to_string());

        if data.find_by_id(uuid.clone()) {
            let mov = data.next::<Transaction>()
                .expect("Clound't parse the transaction");

            if mov.transfer.is_some() {
                data.remove_by_id(mov.transfer.unwrap().uuid);
            }
        }

        data.remove_by_id(uuid);
    }

    // Store the trasaction with validation if is a tranfer
    pub fn make_transaction_or_transfer(storage: &mut Storage, transaction: &mut Transaction, contact_uuid: String) -> String {

        //Transfer
        if transaction.contact.is_none() {

            let mut transfer = transaction.clone();

            // Destination account
            transfer.account = Some(Account::get_account(storage, contact_uuid).unwrap());

            // Update the current transaction with link to
            // transaction of other account
            transaction.transfer = Some(Box::new(transfer.clone()));

            // And link the transaction of the other account
            // with this
            transfer.transfer = Some(Box::new(transaction.clone()));

            return Transaction::store_transfer(storage, transaction, &mut transfer);
        } else {
            return Transaction::store_transaction(storage, transaction.to_owned());
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use uuid::Uuid;
    use std::collections::HashMap;
    use std::cell::RefCell;

    fn populate() -> String {

        I18n::config("en_US".to_string());

        let path = "/tmp/bmoney-".to_owned() + &Uuid::new_v4().to_string();

        let mut st = Storage { path_str: path.clone(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new())  };

        Account::store_account(&mut st, Account { uuid: "".to_string(), name: "account AA".to_string(), bank: "bank A".to_string(), currency: "R$".to_string(), open_balance: 0.0, open_balance_date: Some(Local::today().naive_local()) });
        Account::store_account(&mut st, Account { uuid: "".to_string(), name: "account BB".to_string(), bank: "bank B".to_string(), currency: "R$".to_string(), open_balance: 35.0, open_balance_date: Some(Local::today().naive_local()) });

        let accounts = Account::get_accounts(&mut st);

        Contact::store_contact(&mut st, Contact { uuid: "".to_string(), name: "contact 1".to_string(), city_location: "city A".to_string() });
        Contact::store_contact(&mut st, Contact { uuid: "".to_string(), name: "contact 2".to_string(), city_location: "city B".to_string() });

        let contacts = Contact::get_contacts(&mut st);

        assert!(st.start_section("transactions".to_string()));

        let mut data = st.get_section_data("transactions".to_string());

        data.save(Transaction {
            description: "transaction 1".to_string(),
            value: 10.00,
            account: Some(accounts[0].clone()),
            contact: Some(contacts[0].clone()),
            deadline: Some(NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap()),
            ..Default::default()
        });
        data.save(Transaction {
            description: "transaction 2".to_string(),
            value: -125.53,
            account: Some(accounts[0].clone()),
            contact: Some(contacts[1].clone()),
            deadline: Some(NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap()),
            ..Default::default()
        });
        data.save(Transaction {
            description: "transaction 3".to_string(),
            value: 25.58,
            account: Some(accounts[1].clone()),
            contact: Some(contacts[1].clone()),
            deadline: Some(NaiveDate::parse_from_str("2018-10-15", "%Y-%m-%d").unwrap()),
            ..Default::default()
        });
        data.save(Transaction {
            description: "transaction 4".to_string(),
            value: 159.02,
            account: Some(accounts[0].clone()),
            contact: Some(contacts[0].clone()),
            deadline: Some(NaiveDate::parse_from_str("2018-08-23", "%Y-%m-%d").unwrap()),
            ..Default::default()
        });

        path
    }

    #[test]
    fn get_transactions() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new())  };

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account BB".to_string());

        let (transactions, _total) = Transaction::get_transactions(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, false);

        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0].description, "transaction 2".to_string());
        assert_eq!(transactions[1].description, "transaction 1".to_string());
    }

    #[test]
    fn get_transactions_simple() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new())  };

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account BB".to_string());

        let transactions = Transaction::get_transactions_simple(&mut st, accounts[0].clone());

        assert_eq!(transactions.len(), 3);
        assert_eq!(transactions[0].description, "transaction 4".to_string());
        assert_eq!(transactions[1].description, "transaction 2".to_string());
        assert_eq!(transactions[2].description, "transaction 1".to_string());
    }

    #[test]
    fn get_transactions_totals() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new())  };

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account BB".to_string());
        assert_eq!(accounts[1].name, "account AA".to_string());

        let (transactions_a, totals_a) = Transaction::get_transactions(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, false);

        assert_eq!(transactions_a.len(), 2);

        assert_eq!(totals_a.len(), 9);
        assert_eq!(totals_a[0].label, "Expenses(payable)".to_string());
        assert_eq!(totals_a[0].value, -125.53);
        assert_eq!(totals_a[1].label, "Expenses".to_string());
        assert_eq!(totals_a[1].value, 0.0);
        assert_eq!(totals_a[3].label, "Incomes(to receive)".to_string());
        assert_eq!(totals_a[3].value, 10.0);
        assert_eq!(totals_a[4].label, "Incomes".to_string());
        assert_eq!(totals_a[4].value, 0.0);
        assert_eq!(totals_a[6].label, "Previous balance".to_string());
        assert_eq!(totals_a[6].value, 35.0);
        assert_eq!(totals_a[7].label, "Expected balance".to_string());
        assert_eq!(totals_a[7].value, 78.490005);
        assert_eq!(totals_a[8].label, "Total balance".to_string());
        assert_eq!(totals_a[8].value, 35.0);

        let mut paid = transactions_a[0].clone();
        paid.paid_in = Some(NaiveDate::parse_from_str("2018-10-25", "%Y-%m-%d").unwrap());
        Transaction::store_transaction(&mut st, paid);

        let (transactions_b, totals_b) = Transaction::get_transactions(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, false);

        assert_eq!(transactions_b.len(), 2);

        assert_eq!(totals_b.len(), 9);
        assert_eq!(totals_b[0].label, "Expenses(payable)".to_string());
        assert_eq!(totals_b[0].value, 0.0);
        assert_eq!(totals_b[1].label, "Expenses".to_string());
        assert_eq!(totals_b[1].value, -125.53);
        assert_eq!(totals_b[3].label, "Incomes(to receive)".to_string());
        assert_eq!(totals_b[3].value, 10.0);
        assert_eq!(totals_b[4].label, "Incomes".to_string());
        assert_eq!(totals_b[5].value, 0.0);
        assert_eq!(totals_b[6].label, "Previous balance".to_string());
        assert_eq!(totals_b[6].value, 35.0);
        assert_eq!(totals_b[7].label, "Expected balance".to_string());
        assert_eq!(totals_b[7].value, 78.490005);
        assert_eq!(totals_b[8].label, "Total balance".to_string());
        assert_eq!(totals_b[8].value, -90.53);

        let (transactions_c, totals_c) = Transaction::get_transactions(&mut st, accounts[1].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, false);

        assert_eq!(transactions_c.len(), 1);

        assert_eq!(totals_c.len(), 9);
        assert_eq!(totals_c[0].label, "Expenses(payable)".to_string());
        assert_eq!(totals_c[0].value, 0.0);
        assert_eq!(totals_c[1].label, "Expenses".to_string());
        assert_eq!(totals_c[1].value, 0.0);
        assert_eq!(totals_c[3].label, "Incomes(to receive)".to_string());
        assert_eq!(totals_c[3].value, 25.580002);
        assert_eq!(totals_c[4].label, "Incomes".to_string());
        assert_eq!(totals_c[5].value, 0.0);
        assert_eq!(totals_c[6].label, "Previous balance".to_string());
        assert_eq!(totals_c[6].value, 0.0);
        assert_eq!(totals_c[7].label, "Expected balance".to_string());
        assert_eq!(totals_c[7].value, 25.580002);
        assert_eq!(totals_c[8].label, "Total balance".to_string());
        assert_eq!(totals_c[8].value, 0.0);
    }

    #[test]
    fn transfers() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new())  };

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account BB".to_string());
        assert_eq!(accounts[1].name, "account AA".to_string());

        let contacts = Contact::get_contacts(&mut st);

        assert_eq!(contacts[0].name, "contact 2".to_string());

        let mut from = Transaction {
            description: "transaction from".to_string(),
            value: 20.00,
            account: Some(accounts[0].clone()),
            contact: Some(contacts[0].clone()),
            deadline: Some(NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap()),
            ..Default::default()
        };

        let mut to = from.clone();
        to.account = Some(accounts[1].clone());
        to.description = "transaction to".to_string();

        Transaction::store_transfer(&mut st, &mut from, &mut to);

        let (transactions_a, totals_a) = Transaction::get_transactions(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, false);

        assert_eq!(transactions_a.len(), 3);

        assert_eq!(totals_a.len(), 9);
        assert_eq!(totals_a[3].label, "Incomes(to receive)".to_string());
        assert_eq!(totals_a[3].value, 30.0);

        let (transactions_b, totals_b) = Transaction::get_transactions(&mut st, accounts[1].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, false);

        assert_eq!(transactions_b.len(), 2);

        assert_eq!(totals_b.len(), 9);
        assert_eq!(totals_b[0].label, "Expenses(payable)".to_string());
        assert_eq!(totals_b[0].value, -20.0);

        // Paying the transfer
        let mut paid = transactions_a[0].clone();
        assert_eq!(paid.description, "transaction from".to_string());
        paid.paid_in = Some(NaiveDate::parse_from_str("2018-10-19", "%Y-%m-%d").unwrap());
        Transaction::store_transfer(&mut st, &mut paid.clone(), &mut paid.transfer.unwrap());

        let (transactions_c, totals_c) = Transaction::get_transactions(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, false);

        assert_eq!(transactions_c.len(), 3);

        assert_eq!(totals_c.len(), 9);
        assert_eq!(totals_c[3].label, "Incomes(to receive)".to_string());
        assert_eq!(totals_c[3].value, 10.0);
        assert_eq!(totals_c[5].label, "Transfers in".to_string());
        assert_eq!(totals_c[5].value, 20.0);

        let (transactions_d, totals_d) = Transaction::get_transactions(&mut st, accounts[1].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, false);

        assert_eq!(transactions_d.len(), 2);

        assert_eq!(totals_d.len(), 9);
        assert_eq!(totals_d[0].label, "Expenses(payable)".to_string());
        assert_eq!(totals_d[0].value, 0.0);
        assert_eq!(totals_d[2].label, "Transfers out".to_string());
        assert_eq!(totals_d[2].value, -20.0);
    }

    #[test]
    fn get_transactions_status() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new())  };

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account BB".to_string());

        let (transactions_tmp, _totals) = Transaction::get_transactions(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, false);

        assert_eq!(transactions_tmp.len(), 2);

        let mut paid = transactions_tmp[0].clone();
        paid.paid_in = Some(NaiveDate::parse_from_str("2018-10-25", "%Y-%m-%d").unwrap());
        Transaction::store_transaction(&mut st, paid);

        let (transactions_a, totals_a) = Transaction::get_transactions(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, false);

        assert_eq!(transactions_a.len(), 2);

        assert_eq!(totals_a.len(), 9);
        assert_eq!(totals_a[0].label, "Expenses(payable)".to_string());
        assert_eq!(totals_a[0].value, 0.0);
        assert_eq!(totals_a[1].label, "Expenses".to_string());
        assert_eq!(totals_a[1].value, -125.53);
        assert_eq!(totals_a[3].label, "Incomes(to receive)".to_string());
        assert_eq!(totals_a[3].value, 10.0);
        assert_eq!(totals_a[4].label, "Incomes".to_string());
        assert_eq!(totals_a[4].value, 0.0);
        assert_eq!(totals_a[6].label, "Previous balance".to_string());
        assert_eq!(totals_a[6].value, 35.0);
        assert_eq!(totals_a[7].label, "Expected balance".to_string());
        assert_eq!(totals_a[7].value, 78.490005);
        assert_eq!(totals_a[8].label, "Total balance".to_string());
        assert_eq!(totals_a[8].value, -90.53);

        let (transactions_b, totals_b) = Transaction::get_transactions(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::PAID, None, false);

        assert_eq!(transactions_b.len(), 1);

        assert_eq!(totals_b.len(), 9);
        assert_eq!(totals_b[0].label, "Expenses(payable)".to_string());
        assert_eq!(totals_b[0].value, 0.0);
        assert_eq!(totals_b[1].label, "Expenses".to_string());
        assert_eq!(totals_b[1].value, -125.53);
        assert_eq!(totals_b[3].label, "Incomes(to receive)".to_string());
        assert_eq!(totals_b[3].value, 0.0);
        assert_eq!(totals_b[4].label, "Incomes".to_string());
        assert_eq!(totals_b[4].value, 0.0);
        assert_eq!(totals_b[6].label, "Previous balance".to_string());
        assert_eq!(totals_b[6].value, 35.0);
        assert_eq!(totals_b[7].label, "Expected balance".to_string());
        assert_eq!(totals_b[7].value, -90.53);
        assert_eq!(totals_b[8].label, "Total balance".to_string());
        assert_eq!(totals_b[8].value, -90.53);

        let (transactions_c, totals_c) = Transaction::get_transactions(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::FORPAY, None, false);

        assert_eq!(transactions_c.len(), 1);

        assert_eq!(totals_c.len(), 9);
        assert_eq!(totals_c[0].label, "Expenses(payable)".to_string());
        assert_eq!(totals_c[0].value, 0.0);
        assert_eq!(totals_c[1].label, "Expenses".to_string());
        assert_eq!(totals_c[1].value, 0.0);
        assert_eq!(totals_c[3].label, "Incomes(to receive)".to_string());
        assert_eq!(totals_c[3].value, 10.0);
        assert_eq!(totals_c[4].label, "Incomes".to_string());
        assert_eq!(totals_c[4].value, 0.0);
        assert_eq!(totals_c[6].label, "Previous balance".to_string());
        assert_eq!(totals_c[6].value, 35.0);
        assert_eq!(totals_c[7].label, "Expected balance".to_string());
        assert_eq!(totals_c[7].value, 204.02);
        assert_eq!(totals_c[8].label, "Total balance".to_string());
        assert_eq!(totals_c[8].value, 35.0);
    }

    #[test]
    fn get_transaction() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new())  };

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account BB".to_string());

        let (transactions, _total) = Transaction::get_transactions(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, false);

        let uuid = transactions[0].uuid.clone();

        let transaction = Transaction::get_transaction(&mut st, uuid);

        assert!(transaction.is_ok());
        assert_eq!(transaction.unwrap().description, "transaction 2".to_string());

        let transactione = Transaction::get_transaction(&mut st, "NOOOO".to_string());

        assert!(transactione.is_err());
    }

    #[test]
    fn store_transaction() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new())  };

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account BB".to_string());

        let contacts = Contact::get_contacts(&mut st);

        assert_eq!(contacts[0].name, "contact 2".to_string());

        Transaction::store_transaction(&mut st, Transaction {
            description: "transaction 5".to_string(),
            value: 20.00,
            account: Some(accounts[0].clone()),
            contact: Some(contacts[0].clone()),
            deadline: Some(NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap()),
            ..Default::default()
        });

        let (transactions, _total) = Transaction::get_transactions(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, false);

        assert_eq!(transactions[0].description, "transaction 5".to_string());
        assert_eq!(transactions[1].description, "transaction 2".to_string());
        assert_eq!(transactions[2].description, "transaction 1".to_string());
    }

    #[test]
    fn remove_transaction() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new())  };

        let accounts = Account::get_accounts(&mut st);

        assert_eq!(accounts[0].name, "account BB".to_string());

        let (transactions, _total) = Transaction::get_transactions(&mut st, accounts[0].clone(), NaiveDate::parse_from_str("2018-10-01", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2018-10-31", "%Y-%m-%d").unwrap(), StatusFilter::ALL, None, false);

        let uuid = transactions[0].uuid.clone();

        let transaction = Transaction::get_transaction(&mut st, uuid.clone());

        assert!(transaction.is_ok());

        Transaction::remove_transaction(&mut st, uuid.clone());

        let transactione = Transaction::get_transaction(&mut st, uuid.clone());

        assert!(transactione.is_err());
    }
}
