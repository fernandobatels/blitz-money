///
/// Blitz Money
///
/// Frontend/Ui of module for manange money movimentations
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use chrono::{Local, prelude::Datelike, NaiveDate};

use backend::movimentations::Movimentation;
use backend::movimentations::StatusFilter;
use backend::accounts::Account;
use backend::contacts::Contact;
use backend::tags::Tag;
use backend::storage::Storage;
use ui::ui::*;

pub struct Movimentations {}

impl Movimentations {

    // List of user movimentations
    pub fn list(mut storage: Storage, mut params: Vec<String>, is_csv: bool) {

        if params.len() == 1 || params.len() >= 3 {

            let account = Account::get_account(&mut storage, params[0].trim().to_string()).unwrap();

            let show_all = Input::extract_param(&mut params, "--show-all".to_string());


            let mut status = StatusFilter::ALL;

            if Input::extract_param(&mut params, "--only-forpay".to_string()) {
                status = StatusFilter::FORPAY;
            }

            if Input::extract_param(&mut params, "--only-paid".to_string()) {
                status = StatusFilter::PAID;
            }


            let mut from = Local::now().with_day(1).unwrap().date().naive_local();
            let mut to = Local::now().with_day(30).unwrap().date().naive_local();// yes, fix to get last day of month

            if params.len() == 3 {
                from = NaiveDate::parse_from_str(&params[1].trim().to_string(), "%Y-%m-%d").unwrap();
                to = NaiveDate::parse_from_str(&params[2].trim().to_string(), "%Y-%m-%d").unwrap();
            }


            let (movimentations, totals) = Movimentation::get_movimentations(&mut storage, account.clone(), from, to, status);

            let mut table = Output::new_table();

            if show_all {
                table.set_titles(row![b->"Description", b->"Type", b->"Value", b->"Deadline", b->"Paid in", b->"Contact", b->"Tags", b->"#id", b->"Observations", b->"Created at", b->"Last update"]);
            } else {
                table.set_titles(row![b->"Description", b->"Type", b->"Value", b->"Deadline", b->"Paid in", b->"Contact", b->"Tags", b->"#id"]);
            }

            for movimentation in movimentations {

                let tags: Vec<String> = movimentation.tags
                    .iter()
                    .map(|tag| tag.name.clone())
                    .collect();

                let mut row = table.add_row(row![
                    movimentation.description,
                    "C",
                    Fg->movimentation.value_formmated(),
                    movimentation.deadline.unwrap(),
                    movimentation.paid_in_formmated(),
                    "",
                    tags.join(", "),
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

                    // In trascations we show the destination account on place of contact
                    row.set_cell(cell!(movimentation.transaction.unwrap().account.clone().unwrap().name + &"(account)".to_owned()), 5)
                        .expect("Unable to set account of other transaction on movimentation");
                } else {
                    row.set_cell(cell!(movimentation.contact.clone().unwrap().name), 5)
                        .expect("Unable to set contact on movimentation");
                }

                if show_all {
                    row.add_cell(cell!(movimentation.observations));
                    row.add_cell(cell!(movimentation.created_at.unwrap()));
                    if movimentation.updated_at.is_some() {
                        row.add_cell(cell!(movimentation.updated_at.unwrap()));
                    } else {
                        row.add_cell(cell!(""));
                    }
                }
            }


            if show_all {
                table.add_row(row!["", "", "", "", "", "", "", "", "", "", ""]);
            } else {
                table.add_row(row!["", "", "", "", "", "", "", ""]);
            }

            for total in totals {

                let mut row = table.add_row(row![
                    b->total.label,
                    "",
                    Fg->account.format_value(total.value),
                    "",
                    "",
                    "",
                    "",
                    ""
                ]);

                if total.value < 0.0 {
                    row.set_cell(cell!(Fr->account.format_value(total.value)), 2)
                        .expect("Unable to set value on total");
                }

                if show_all {
                    row.add_cell(cell!(""));
                    row.add_cell(cell!(""));
                    row.add_cell(cell!(""));
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

        if params.len() >= 5 || (params.len() == 1 && params[0] == "-i") {

            let description: String;
            let account: Option<Account>;
            let value: f32;
            let contact_uuid;
            let deadline: Option<NaiveDate>;
            let paid_in: Option<NaiveDate>;
            let mut tags: Vec<Tag> = vec!();
            let observations: String;

            if params.len() >= 5 {
                // Shell mode

                description = Input::param("Movimentation description".to_string(), true, params.clone(), 0);

                let account_uuid = Input::param("Account".to_string(), true, params.clone(), 2);
                account = Some(Account::get_account(&mut storage, account_uuid).unwrap());

                value = Input::param_money("Value(>= 0 for credit and < 0 for debit)".to_string(), true, params.clone(), 1);

                contact_uuid = Input::param("Contact".to_string(), true, params.clone(), 3);

                deadline = Input::param_date("Deadline".to_string(), true, params.clone(), 4);
                paid_in = Input::param_date("Paid in".to_string(), false, params.clone(), 5);

                let tags_str = Input::param("Tags".to_string(), false, params.clone(), 6);

                if !tags_str.is_empty() {
                    for tag in tags_str.split(",") {
                        tags.push(
                            Tag::get_tag(&mut storage, tag.to_string())
                                .expect("Tag not found")
                        );
                    }
                }

                observations = Input::param("Observations".to_string(), false, params.clone(), 7);
            } else {
                // Interactive mode

                description = Input::read("Movimentation description".to_string(), true, None);

                let mut accounts: Vec<(String, String)> = vec![];
                for ac in Account::get_accounts(&mut storage) {
                    accounts.push((ac.uuid, ac.name));
                }

                let account_uuid = Input::read_option("Account".to_string(), true, None, accounts.clone());
                account = Some(Account::get_account(&mut storage, account_uuid).unwrap());

                value = Input::read_money("Value(>= 0 for credit and < 0 for debit)".to_string(), true, None, account.clone().unwrap().currency);

                let mut contacts: Vec<(String, String)> = vec![];
                for co in Contact::get_contacts(&mut storage) {
                    contacts.push((co.uuid, co.name));
                }
                // For transactions
                for (uuid, name) in accounts {
                    contacts.push((uuid, name + &"(account)".to_owned()));
                }

                contact_uuid = Input::read_option("Contact or other account(for transaction)".to_string(), true, None, contacts);

                deadline = Input::read_date("Deadline".to_string(), true, None);
                paid_in = Input::read_date("Paid in".to_string(), false, None);

                let mut tags_ops: Vec<(String, String)> = vec![];
                for tag in Tag::get_tags(&mut storage) {
                    tags_ops.push((tag.uuid, tag.name));
                }

                tags = Input::read_options("Tags".to_string(), false, vec![], tags_ops)
                    .iter()
                    .map(
                        |tag| Tag::get_tag(&mut storage, tag.to_string())
                                    .expect("Tag not found")
                    )
                    .collect();

                observations = Input::read("Observations".to_string(), false, None);
            }

            let contact = match Contact::get_contact(&mut storage, contact_uuid.clone()) {
                Ok(con) => Some(con),
                Err(_)  => None
            };

            let mut mov = Movimentation {
                description: description,
                value: value,
                account: account,
                contact: contact.clone(),
                deadline: deadline,
                paid_in: paid_in,
                tags: tags,
                observations: observations,
                ..Default::default()
            };

            //Transaction
            if contact.is_none() {

                let mut transaction = mov.clone();

                // Destination account
                transaction.account = Some(Account::get_account(&mut storage, contact_uuid).unwrap());

                // Update the current movimentation with link to
                // movimentation of other account
                mov.transaction = Some(Box::new(transaction.clone()));

                // And link the movimentation of the other account
                // with this
                transaction.transaction = Some(Box::new(mov.clone()));

                Movimentation::store_transaction(&mut storage, &mut mov, &mut transaction);
            } else {
                Movimentation::store_movimentation(&mut storage, mov);
            }

        } else {
            // Help mode
            println!("How to use: bmoney movimentations add [description] [value] [account id] [contact id] [deadline] [paid in] [tags] [observations]");
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

            if movimentation.transaction.is_some() {
                Movimentation::store_transaction(&mut storage, &mut movimentation.clone(), &mut movimentation.transaction.unwrap());
            } else {
                Movimentation::store_movimentation(&mut storage, movimentation);
            }

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
                if movimentation.transaction.is_some() {
                    movimentation.account = Some(Account::get_account(&mut storage, contact_uuid).unwrap());
                } else {
                    movimentation.contact = Some(Contact::get_contact(&mut storage, contact_uuid).unwrap());
                }
            } else if params[1] == "deadline" {
                movimentation.deadline = Input::param_date("Deadline".to_string(), true, params.clone(), 2);
            } else if params[1] == "paid_in" {
                movimentation.paid_in = Input::param_date("Paid in".to_string(), false, params.clone(), 2);
            } else if params[1] == "tags" {
                let tags_str = Input::param("Tags".to_string(), false, params.clone(), 2);
                movimentation.tags = vec![];

                if !tags_str.is_empty() {
                    for tag in tags_str.split(",") {
                        movimentation.tags.push(
                            Tag::get_tag(&mut storage, tag.to_string())
                                .expect("Tag not found")
                        );
                    }
                }
            } else if params[1] == "observations" {
                movimentation.observations = Input::param("Observations".to_string(), false, params.clone(), 2);
            } else {
                panic!("Field not found!");
            }

            if movimentation.transaction.is_some() {
                Movimentation::store_transaction(&mut storage, &mut movimentation.clone(), &mut movimentation.transaction.unwrap());
            } else {
                Movimentation::store_movimentation(&mut storage, movimentation);
            }

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

            let account_uuid = Input::read_option("Account".to_string(), true, Some(movimentation.account.clone().unwrap().uuid), accounts.clone());
            movimentation.account = Some(Account::get_account(&mut storage, account_uuid).unwrap());

            movimentation.value = Input::read_money("Value(>= 0 for credit and < 0 for debit)".to_string(), true, Some(movimentation.value), movimentation.account.clone().unwrap().currency);

            let contact_uuid: String;

            if movimentation.transaction.is_some() {
                contact_uuid = Input::read_option("Destination account".to_string(), true, Some(movimentation.transaction.clone().unwrap().uuid), accounts);
            } else {
                let mut contacts: Vec<(String, String)> = vec![];
                for co in Contact::get_contacts(&mut storage) {
                    contacts.push((co.uuid, co.name));
                }

                contact_uuid = Input::read_option("Contact".to_string(), true, Some(movimentation.contact.clone().unwrap().uuid), contacts);
                movimentation.contact = Some(Contact::get_contact(&mut storage, contact_uuid.clone()).unwrap());
            }

            movimentation.deadline = Input::read_date("Deadline".to_string(), true, movimentation.deadline);
            movimentation.paid_in = Input::read_date("Paid in".to_string(), false, movimentation.paid_in);

            let mut tags_ops: Vec<(String, String)> = vec![];
            for tag in Tag::get_tags(&mut storage) {
                tags_ops.push((tag.uuid, tag.name));
            }

            let current_tags: Vec<String> = movimentation.tags.clone()
                .iter()
                .map(|tag| tag.uuid.clone())
                .collect();

            movimentation.tags = Input::read_options("Tags".to_string(), false, current_tags, tags_ops)
                .iter()
                .map(
                    |tag| Tag::get_tag(&mut storage, tag.to_string())
                                .expect("Tag not found")
                )
                .collect();

            movimentation.observations = Input::read("Observations".to_string(), false, Some(movimentation.observations));

            if movimentation.transaction.is_some() {
                let mut transaction = movimentation.clone().transaction.unwrap();

                // Destination account
                transaction.account = Some(Account::get_account(&mut storage, contact_uuid).unwrap());

                Movimentation::store_transaction(&mut storage, &mut movimentation, &mut transaction);
            } else {
                Movimentation::store_movimentation(&mut storage, movimentation);
            }
        } else {
            // Help mode
            println!("How to use: bmoney movimentations update [id] [description|value|account|contact|deadline|paid|tags|observations] [value]");
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
