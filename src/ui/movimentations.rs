///
/// Blitz Money
///
/// Frontend/Ui of module for manange money movimentations
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use chrono::{Local, prelude::Datelike, NaiveDate};
use std::io;

use backend::movimentations::Movimentation;
use backend::accounts::Account;
use backend::contacts::Contact;
use backend::storage::Storage;
use ui::ui::Output;

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

            table.set_titles(row![b->"Description", b->"Value", b->"Deadline", b->"Paid in", b->"Contact", b->"#id"]);

            for movimentation in movimentations {

                if movimentation.value >= 0.0 {
                    table.add_row(row![
                        movimentation.description,
                        Fg->movimentation.value_formmated(),
                        movimentation.deadline.unwrap(),
                        movimentation.paid_in_formmated(),
                        movimentation.contact.unwrap().name,
                        movimentation.uuid
                    ]);
                } else {
                    table.add_row(row![
                        movimentation.description,
                        Fr->movimentation.value_formmated(),
                        movimentation.deadline.unwrap(),
                        movimentation.paid_in_formmated(),
                        movimentation.contact.unwrap().name,
                        movimentation.uuid
                    ]);
                }
            }


            table.add_row(row!["", "", "", "", "", ""]);

            for total in totals {

                if total.value >= 0.0 {
                    table.add_row(row![
                        b->total.label,
                        Fg->account.format_value(total.value),
                        "",
                        "",
                        "",
                        ""
                    ]);
                } else {
                    table.add_row(row![
                        b->total.label,
                        Fr->account.format_value(total.value),
                        "",
                        "",
                        "",
                        ""
                    ]);
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
            let account = Some(Account::get_account(&mut storage, params[2].trim().to_string()).unwrap());
            let contact = Some(Contact::get_contact(&mut storage, params[3].trim().to_string()).unwrap());

            Movimentation::store_movimentation(&mut storage, Movimentation {
                uuid: "".to_string(),
                description: params[0].trim().to_string(),
                value: params[1].trim().parse::<f32>().unwrap(),
                account: account,
                contact: contact,
                deadline: Some(NaiveDate::parse_from_str(&params[4].trim().to_string(), "%Y-%m-%d").unwrap()),
                paid_in: params[5].trim().to_string(),
                created_at: Some(Local::now()),
            });
        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            println!("Movimentation description:");
            let mut description = String::new();
            io::stdin().read_line(&mut description)
                .expect("Failed to read description");

            println!("Value(>= 0 for credit and < 0 for debit):");
            let mut value = String::new();
            io::stdin().read_line(&mut value)
                .expect("Failed to read value");

            println!("Account:");
            let mut account_uuid = String::new();
            io::stdin().read_line(&mut account_uuid)
                .expect("Failed to read account");
            let account = Some(Account::get_account(&mut storage, account_uuid.trim().to_string()).unwrap());

            println!("Contact:");
            let mut contact_uuid = String::new();
            io::stdin().read_line(&mut contact_uuid)
                .expect("Failed to read contact");
            let contact = Some(Contact::get_contact(&mut storage, contact_uuid.trim().to_string()).unwrap());

            println!("Deadline(format YYYY-MM-DD):");
            let mut deadline = String::new();
            io::stdin().read_line(&mut deadline)
                .expect("Failed to read deadline");

            println!("Paid in(format YYYY-MM-DD):");
            let mut paid_in = String::new();
            io::stdin().read_line(&mut paid_in)
                .expect("Failed to read paid in");

            Movimentation::store_movimentation(&mut storage, Movimentation {
                uuid: "".to_string(),
                description: description.trim().to_string(),
                value: value.trim().parse::<f32>().unwrap(),
                account: account,
                contact: contact,
                deadline: Some(NaiveDate::parse_from_str(&deadline.trim().to_string(), "%Y-%m-%d").unwrap()),
                paid_in: paid_in.trim().to_string(),
                created_at: Some(Local::now()),
            });
        } else {
            // Help mode
            println!("How to use: bmoney movimentations add [description] [value] [account id] [contact id] [deadline] [paid in]");
            println!("Or with interactive mode: bmoney movimentations add -i")
        }
    }

    // Update a existing movimentation
    pub fn update(mut storage: Storage, params: Vec<String>) {

        if params.len() == 3 {
            // Shell mode

            let mut movimentation = Movimentation::get_movimentation(&mut storage, params[0].trim().to_string())
                .expect("Movimentation not found");

            if params[1] == "description" {
                movimentation.description = params[2].trim().to_string();
            } else if params[1] == "value" {
                movimentation.value = params[2].trim().parse::<f32>().unwrap();
            } else if params[1] == "account" {
                movimentation.account = Some(Account::get_account(&mut storage, params[2].trim().to_string()).unwrap());
            } else if params[1] == "contact" {
                movimentation.contact = Some(Contact::get_contact(&mut storage, params[2].trim().to_string()).unwrap());
            } else if params[1] == "deadline" {
                movimentation.deadline = Some(NaiveDate::parse_from_str(&params[2].trim().to_string(), "%Y-%m-%d").unwrap());
            } else if params[1] == "paid_in" {
                movimentation.paid_in = params[2].trim().to_string();
            } else {
                panic!("Field not found!");
            }

            Movimentation::store_movimentation(&mut storage, movimentation);

        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            println!("Movimentation id:");
            let mut id = String::new();
            io::stdin().read_line(&mut id)
                .expect("Failed to read id");

            let mut movimentation = Movimentation::get_movimentation(&mut storage, id.trim().to_string())
                .expect("Movimentation not found");

            println!("Movimentation description: {}(keep blank for skip)", movimentation.description);
            let mut description = String::new();
            io::stdin().read_line(&mut description)
                .expect("Failed to read description");
            if !description.trim().is_empty() {
                movimentation.description = description.trim().to_string();
            }

            println!("Value: {}(keep blank for skip)", movimentation.value);
            let mut value = String::new();
            io::stdin().read_line(&mut value)
                .expect("Failed to read value");
            if !value.trim().is_empty() {
                movimentation.value = value.trim().parse::<f32>().unwrap();
            }

            println!("Account: {}(keep blank for skip)", movimentation.account.clone().unwrap().name);
            let mut account = String::new();
            io::stdin().read_line(&mut account)
                .expect("Failed to read account");
            if !account.trim().is_empty() {
                movimentation.account = Some(Account::get_account(&mut storage, account.trim().to_string()).unwrap());
            }

            println!("Contact: {}(keep blank for skip)", movimentation.contact.clone().unwrap().name);
            let mut contact = String::new();
            io::stdin().read_line(&mut contact)
                .expect("Failed to read contact");
            if !contact.trim().is_empty() {
                movimentation.contact = Some(Contact::get_contact(&mut storage, contact.trim().to_string()).unwrap());
            }

            println!("Deadline(format YYYY-MM-DD): {}(keep blank for skip)", movimentation.deadline.unwrap());
            let mut deadline = String::new();
            io::stdin().read_line(&mut deadline)
                .expect("Failed to read deadline");
            if !deadline.trim().is_empty() {
                movimentation.deadline = Some(NaiveDate::parse_from_str(&deadline.trim().to_string(), "%Y-%m-%d").unwrap());
            }

            println!("Paid in(format YYYY-MM-DD): {}(keep blank for skip)", movimentation.paid_in);
            let mut paid_in = String::new();
            io::stdin().read_line(&mut paid_in)
                .expect("Failed to read paid_in");
            if !paid_in.trim().is_empty() {
                movimentation.paid_in = paid_in.trim().to_string();
            }

            Movimentation::store_movimentation(&mut storage, movimentation);
        } else {
            // Help mode
            println!("How to use: bmoney movimentations update [id] [description|value|account|contact|deadline|paid] [value]");
            println!("Or with interactive mode: bmoney movimentations update -i")
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
