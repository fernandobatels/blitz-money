///
/// Blitz Money
///
/// Frontend/Ui of module for manange money movimentations
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use chrono::{Local, prelude::Datelike, NaiveDate};

use backend::movimentations::Movimentation;
use backend::accounts::Account;
use backend::contacts::Contact;
use backend::storage::Storage;
use ui::ui::*;

pub struct Movimentations {}

impl Movimentations {

    // List of user movimentations
    pub fn list(mut storage: Storage, params: Vec<String>, is_csv: bool) {

        if params.len() == 1 || params.len() == 3 {

            let account = Account::get_account(&mut storage, params[0].trim().to_string()).unwrap();

            let mut from = Local::now().with_day(1).unwrap().date().naive_local();
            let mut to = Local::now().with_day(30).unwrap().date().naive_local();// yes, fix to get last day of month

            if params.len() == 3 {
                from = NaiveDate::parse_from_str(&params[1].trim().to_string(), "%Y-%m-%d").unwrap();
                to = NaiveDate::parse_from_str(&params[2].trim().to_string(), "%Y-%m-%d").unwrap();
            }

            let (movimentations, totals) = Movimentation::get_movimentations(&mut storage, account.clone(), from, to);

            let mut table = Output::new_table();

            table.set_titles(row![b->"Description", b->"Type", b->"Value", b->"Deadline", b->"Paid in", b->"Contact", b->"#id"]);

            for movimentation in movimentations {

                let mut row = table.add_row(row![
                    movimentation.description,
                    "C",
                    Fg->movimentation.value_formmated(),
                    movimentation.deadline.unwrap(),
                    movimentation.paid_in_formmated(),
                    movimentation.contact.clone().unwrap().name,
                    movimentation.uuid
                ]);

                if movimentation.value < 0.0 {
                    row.set_cell(cell!("D"), 1)
                        .expect("Unable to set D on movimentation");
                    row.set_cell(cell!(Fr->movimentation.value_formmated()), 2)
                        .expect("Unable to set value on movimentation");
                }

                if movimentation.transaction.is_some() {
                    row.set_cell(cell!("T"), 1)
                        .expect("Unable to set T on movimentation");
                }
            }


            table.add_row(row!["", "", "", "", "", "", ""]);

            for total in totals {

                let mut row = table.add_row(row![
                    b->total.label,
                    "",
                    Fg->account.format_value(total.value),
                    "",
                    "",
                    "",
                    ""
                ]);

                if total.value < 0.0 {
                    row.set_cell(cell!(Fr->account.format_value(total.value)), 2)
                        .expect("Unable to set value on total");
                }
            }

            Output::print_table(table, is_csv);
        } else {
            // Help mode
            println!("How to use: bmoney movimentations list [account id] [from] [to]");
        }
    }

    // Create new movimentation
    pub fn add(mut storage: Storage, params: Vec<String>) {

        if params.len() == 6 {
            // Shell mode

            let description = Input::param("Movimentation description".to_string(), true, params.clone(), 0);

            let account_uuid = Input::param("Account".to_string(), true, params.clone(), 2);
            let account = Some(Account::get_account(&mut storage, account_uuid).unwrap());

            let value = Input::param_money("Value(>= 0 for credit and < 0 for debit)".to_string(), true, params.clone(), 1);

            let contact_uuid = Input::param("Contact".to_string(), true, params.clone(), 3);
            let contact = Some(Contact::get_contact(&mut storage, contact_uuid).unwrap());

            let deadline = Input::param_date("Deadline".to_string(), true, params.clone(), 4);
            let paid_in = Input::param_date("Paid in".to_string(), false, params.clone(), 5);

            Movimentation::store_movimentation(&mut storage, Movimentation {
                description: description,
                value: value,
                account: account,
                contact: contact,
                deadline: deadline,
                paid_in: paid_in,
                ..Default::default()
            });
        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let description = Input::read("Movimentation description".to_string(), true, None);

            let mut accounts: Vec<(String, String)> = vec![];
            for ac in Account::get_accounts(&mut storage) {
                accounts.push((ac.uuid, ac.name));
            }

            let account_uuid = Input::read_option("Account".to_string(), true, None, accounts);
            let account = Some(Account::get_account(&mut storage, account_uuid).unwrap());

            let value = Input::read_money("Value(>= 0 for credit and < 0 for debit)".to_string(), true, None, account.clone().unwrap().currency);


            let mut contacts: Vec<(String, String)> = vec![];
            for ac in Contact::get_contacts(&mut storage) {
                contacts.push((ac.uuid, ac.name));
            }

            let contact_uuid = Input::read_option("Contact".to_string(), true, None, contacts);
            let contact = Some(Contact::get_contact(&mut storage, contact_uuid).unwrap());

            let deadline = Input::read_date("Deadline".to_string(), true, None);
            let paid_in = Input::read_date("Paid in".to_string(), false, None);

            Movimentation::store_movimentation(&mut storage, Movimentation {
                description: description,
                value: value,
                account: account,
                contact: contact,
                deadline: deadline,
                paid_in: paid_in,
                ..Default::default()
            });
        } else {
            // Help mode
            println!("How to use: bmoney movimentations add [description] [value] [account id] [contact id] [deadline] [paid in]");
            println!("Or with interactive mode: bmoney movimentations add -i")
        }
    }

    // Update a existing movimentation
    pub fn update(mut storage: Storage, params: Vec<String>) {

        if params.len() >= 2 && params[1] == "pay" {
            // Pay mode

            let mut movimentation = Movimentation::get_movimentation(&mut storage, params[0].trim().to_string())
                .expect("Movimentation not found");

            // Update the paid date
            if params.len() >= 3 && params[2] != "today" {
                if params[2].is_empty() || params[2] == "unpaid" {
                    movimentation.paid_in = None;
                } else {
                    // Is a date value
                    movimentation.paid_in = Some(NaiveDate::parse_from_str(&params[2], "%Y-%m-%d")
                        .expect("Couldn't parse the string to date. The format is YYYY-MM-DD"));
                }
            } else {
                // No date, no empty or the 'today' value we set this movimentation as paid today
                movimentation.paid_in = Some(Local::today().naive_local());
            }

            // Update the value
            if params.len() >= 4 {
                movimentation.value = params[3].parse::<f32>()
                    .expect("Couldn't parse the string to money. The format is 00000.00");
            }

            Movimentation::store_movimentation(&mut storage, movimentation);

        } else if params.len() == 3 {
            // Shell mode

            let mut movimentation = Movimentation::get_movimentation(&mut storage, params[0].trim().to_string())
                .expect("Movimentation not found");

            if params[1] == "description" {
                movimentation.description = Input::param("Movimentation description".to_string(), true, params.clone(), 2);
            } else if params[1] == "value" {
                movimentation.value = Input::param_money("Value(>= 0 for credit and < 0 for debit)".to_string(), true, params.clone(), 2);
            } else if params[1] == "account" {
                let account_uuid = Input::param("Account".to_string(), true, params.clone(), 2);
                movimentation.account = Some(Account::get_account(&mut storage, account_uuid).unwrap());
            } else if params[1] == "contact" {
                let contact_uuid = Input::param("Contact".to_string(), true, params.clone(), 2);
                movimentation.contact = Some(Contact::get_contact(&mut storage, contact_uuid).unwrap());
            } else if params[1] == "deadline" {
                movimentation.deadline = Input::param_date("Deadline".to_string(), true, params.clone(), 2);
            } else if params[1] == "paid_in" {
                movimentation.paid_in = Input::param_date("Paid in".to_string(), false, params.clone(), 2);
            } else {
                panic!("Field not found!");
            }

            Movimentation::store_movimentation(&mut storage, movimentation);

        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let id = Input::read("Movimentation id".to_string(), true, None);

            let mut movimentation = Movimentation::get_movimentation(&mut storage, id)
                .expect("Movimentation not found");

            movimentation.description = Input::read("Movimentation description".to_string(), true, Some(movimentation.description));

            let mut accounts: Vec<(String, String)> = vec![];
            for ac in Account::get_accounts(&mut storage) {
                accounts.push((ac.uuid, ac.name));
            }

            let account_uuid = Input::read_option("Account".to_string(), true, Some(movimentation.account.clone().unwrap().uuid), accounts);
            movimentation.account = Some(Account::get_account(&mut storage, account_uuid).unwrap());

            movimentation.value = Input::read_money("Value(>= 0 for credit and < 0 for debit)".to_string(), true, Some(movimentation.value), movimentation.account.clone().unwrap().currency);

            let mut contacts: Vec<(String, String)> = vec![];
            for ac in Contact::get_contacts(&mut storage) {
                contacts.push((ac.uuid, ac.name));
            }

            let contact_uuid = Input::read_option("Contact".to_string(), true, Some(movimentation.contact.clone().unwrap().uuid), contacts);
            movimentation.contact = Some(Contact::get_contact(&mut storage, contact_uuid).unwrap());

            movimentation.deadline = Input::read_date("Deadline".to_string(), true, movimentation.deadline);
            movimentation.paid_in = Input::read_date("Paid in".to_string(), false, movimentation.paid_in);

            Movimentation::store_movimentation(&mut storage, movimentation);
        } else {
            // Help mode
            println!("How to use: bmoney movimentations update [id] [description|value|account|contact|deadline|paid] [value]");
            println!("Or with interactive mode: bmoney movimentations update -i");
            println!("Or for pay mode: bmoney movimentations update [id] pay [\"\"|YYYY-MM-DD|unpaid|today](optional) [new_value](optional)");
        }
    }

    // Remove a existing movimentation
    pub fn rm(mut storage: Storage, params: Vec<String>) {

        if params.len() == 1 {
            // Shell mode

            Movimentation::remove_movimentation(&mut storage, params[0].trim().to_string());
        } else {
            // Help mode
            println!("How to use: bmoney movimentations rm [id]");
        }
    }
}
