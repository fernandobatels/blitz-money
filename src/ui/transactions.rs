///
/// Blitz Money
///
/// Frontend/Ui of module for manange money transactions
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use chrono::{Local, prelude::Datelike, NaiveDate};

use backend::transactions::Transaction;
use backend::transactions::StatusFilter;
use backend::accounts::Account;
use backend::contacts::Contact;
use backend::tags::Tag;
use backend::storage::Storage;
use backend::ofx::Ofx;
use ui::ui::*;

pub struct Transactions {}

impl Transactions {

    // List of user transactions
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


            let uuid = Input::extract_named_param(&mut params, "--uuid=".to_string());


            let tag = Input::extract_named_param(&mut params, "--tag=".to_string());

            let mut from = Local::now().with_day(1).unwrap().date().naive_local();
            let mut to = Local::now().with_day(30).unwrap().date().naive_local();// yes, fix to get last day of month

            if params.len() == 3 {
                from = NaiveDate::parse_from_str(&params[1].trim().to_string(), "%Y-%m-%d").unwrap();
                to = NaiveDate::parse_from_str(&params[2].trim().to_string(), "%Y-%m-%d").unwrap();
            }

            let (transactions, totals) = Transaction::get_transactions(&mut storage, account.clone(), from, to, status, uuid, tag);

            let mut table = Output::new_table();

            if show_all {
                table.set_titles(row![b->"Description", b->"Type", b->"Value", b->"Deadline", b->"Paid in", b->"Contact", b->"Tags", b->"#id", b->"By ofx", b->"Observations", b->"Created at", b->"Last update"]);
            } else {
                table.set_titles(row![b->"Description", b->"Type", b->"Value", b->"Deadline", b->"Paid in", b->"Contact", b->"Tags", b->"#id", b->"By ofx"]);
            }

            for transaction in transactions {

                let tags: Vec<String> = transaction.tags
                    .iter()
                    .map(|tag| tag.name.clone())
                    .collect();

                let mut by_ofx = "No".to_string();
                if !transaction.ofx_fitid.is_empty() {
                    by_ofx = "Yes".to_string();
                }

                let mut row = table.add_row(row![
                    transaction.description,
                    "C",
                    Fg->transaction.value_formmated(),
                    transaction.deadline.unwrap(),
                    transaction.paid_in_formmated(),
                    "",
                    tags.join(", "),
                    transaction.uuid,
                    by_ofx
                ]);

                if transaction.value < 0.0 {
                    row.set_cell(cell!("D"), 1)
                        .expect("Unable to set D on transaction");
                    row.set_cell(cell!(Fr->transaction.value_formmated()), 2)
                        .expect("Unable to set value on transaction");
                }

                if transaction.transfer.is_some() {
                    row.set_cell(cell!("T"), 1)
                        .expect("Unable to set T on transaction");

                    // In transcations we show the destination account on place of contact
                    row.set_cell(cell!(transaction.transfer.unwrap().account.clone().unwrap().name + &"(account)".to_owned()), 5)
                        .expect("Unable to set account of other transfer on transaction");
                } else {
                    row.set_cell(cell!(transaction.contact.clone().unwrap().name), 5)
                        .expect("Unable to set contact on transaction");
                }

                if show_all {
                    row.add_cell(cell!(transaction.observations));
                    row.add_cell(cell!(transaction.created_at.unwrap()));
                    if transaction.updated_at.is_some() {
                        row.add_cell(cell!(transaction.updated_at.unwrap()));
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
            println!("How to use: bmoney transactions list [account id] [from] [to]");
        }
    }

    // Create new transaction
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

                description = Input::param("Transaction description".to_string(), true, params.clone(), 0);

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

                description = Input::read("Transaction description".to_string(), true, None);

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
                // For transfers
                for (uuid, name) in accounts {
                    contacts.push((uuid, name + &"(account)".to_owned()));
                }

                contact_uuid = Input::read_option("Contact or other account(for transfer)".to_string(), true, None, contacts);

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

            let mut mov = Transaction {
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

            Transaction::make_transaction_or_transfer(&mut storage, &mut mov, contact_uuid);

        } else {
            // Help mode
            println!("How to use: bmoney transactions add [description] [value] [account id] [contact id] [deadline] [paid in] [tags] [observations]");
            println!("Or with interactive mode: bmoney transactions add -i")
        }
    }

    // Update a existing transaction
    pub fn update(mut storage: Storage, params: Vec<String>) {

        if params.len() >= 2 && params[1] == "pay" {
            // Pay mode

            let mut transaction = Transaction::get_transaction(&mut storage, params[0].trim().to_string())
                .expect("Transaction not found");

            // Update the paid date
            if params.len() >= 3 && params[2] != "today" {
                if params[2].is_empty() || params[2] == "unpaid" {
                    transaction.paid_in = None;
                } else {
                    // Is a date value
                    transaction.paid_in = Some(NaiveDate::parse_from_str(&params[2], "%Y-%m-%d")
                        .expect("Couldn't parse the string to date. The format is YYYY-MM-DD"));
                }
            } else {
                // No date, no empty or the 'today' value we set this transaction as paid today
                transaction.paid_in = Some(Local::today().naive_local());
            }

            // Update the value
            if params.len() >= 4 {
                transaction.value = params[3].parse::<f32>()
                    .expect("Couldn't parse the string to money. The format is 00000.00");
            }

            if transaction.transfer.is_some() {
                Transaction::store_transfer(&mut storage, &mut transaction.clone(), &mut transaction.transfer.unwrap());
            } else {
                Transaction::store_transaction(&mut storage, transaction);
            }

        } else if params.len() == 3 {
            // Shell mode

            let mut transaction = Transaction::get_transaction(&mut storage, params[0].trim().to_string())
                .expect("Transaction not found");

            if params[1] == "description" {
                transaction.description = Input::param("Transaction description".to_string(), true, params.clone(), 2);
            } else if params[1] == "value" {
                transaction.value = Input::param_money("Value(>= 0 for credit and < 0 for debit)".to_string(), true, params.clone(), 2);
            } else if params[1] == "account" {
                let account_uuid = Input::param("Account".to_string(), true, params.clone(), 2);
                transaction.account = Some(Account::get_account(&mut storage, account_uuid).unwrap());
            } else if params[1] == "contact" {
                let contact_uuid = Input::param("Contact".to_string(), true, params.clone(), 2);
                if transaction.transfer.is_some() {
                    transaction.account = Some(Account::get_account(&mut storage, contact_uuid).unwrap());
                } else {
                    transaction.contact = Some(Contact::get_contact(&mut storage, contact_uuid).unwrap());
                }
            } else if params[1] == "deadline" {
                transaction.deadline = Input::param_date("Deadline".to_string(), true, params.clone(), 2);
            } else if params[1] == "paid_in" {
                transaction.paid_in = Input::param_date("Paid in".to_string(), false, params.clone(), 2);
            } else if params[1] == "tags" {
                let tags_str = Input::param("Tags".to_string(), false, params.clone(), 2);
                transaction.tags = vec![];

                if !tags_str.is_empty() {
                    for tag in tags_str.split(",") {
                        transaction.tags.push(
                            Tag::get_tag(&mut storage, tag.to_string())
                                .expect("Tag not found")
                        );
                    }
                }
            } else if params[1] == "observations" {
                transaction.observations = Input::param("Observations".to_string(), false, params.clone(), 2);
            } else {
                panic!("Field not found!");
            }

            if transaction.transfer.is_some() {
                Transaction::store_transfer(&mut storage, &mut transaction.clone(), &mut transaction.transfer.unwrap());
            } else {
                Transaction::store_transaction(&mut storage, transaction);
            }

        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let id = Input::read("Transaction id".to_string(), true, None);

            let mut transaction = Transaction::get_transaction(&mut storage, id)
                .expect("Transaction not found");

            transaction.description = Input::read("Transaction description".to_string(), true, Some(transaction.description));

            let mut accounts: Vec<(String, String)> = vec![];
            for ac in Account::get_accounts(&mut storage) {
                accounts.push((ac.uuid, ac.name));
            }

            let account_uuid = Input::read_option("Account".to_string(), true, Some(transaction.account.clone().unwrap().uuid), accounts.clone());
            transaction.account = Some(Account::get_account(&mut storage, account_uuid).unwrap());

            transaction.value = Input::read_money("Value(>= 0 for credit and < 0 for debit)".to_string(), true, Some(transaction.value), transaction.account.clone().unwrap().currency);

            let contact_uuid: String;

            if transaction.transfer.is_some() {
                contact_uuid = Input::read_option("Destination account".to_string(), true, Some(transaction.transfer.clone().unwrap().uuid), accounts);
            } else {
                let mut contacts: Vec<(String, String)> = vec![];
                for co in Contact::get_contacts(&mut storage) {
                    contacts.push((co.uuid, co.name));
                }

                contact_uuid = Input::read_option("Contact".to_string(), true, Some(transaction.contact.clone().unwrap().uuid), contacts);
                transaction.contact = Some(Contact::get_contact(&mut storage, contact_uuid.clone()).unwrap());
            }

            transaction.deadline = Input::read_date("Deadline".to_string(), true, transaction.deadline);
            transaction.paid_in = Input::read_date("Paid in".to_string(), false, transaction.paid_in);

            let mut tags_ops: Vec<(String, String)> = vec![];
            for tag in Tag::get_tags(&mut storage) {
                tags_ops.push((tag.uuid, tag.name));
            }

            let current_tags: Vec<String> = transaction.tags.clone()
                .iter()
                .map(|tag| tag.uuid.clone())
                .collect();

            transaction.tags = Input::read_options("Tags".to_string(), false, current_tags, tags_ops)
                .iter()
                .map(
                    |tag| Tag::get_tag(&mut storage, tag.to_string())
                                .expect("Tag not found")
                )
                .collect();

            transaction.observations = Input::read("Observations".to_string(), false, Some(transaction.observations));

            if transaction.transfer.is_some() {
                let mut transfer = transaction.clone().transfer.unwrap();

                // Destination account
                transfer.account = Some(Account::get_account(&mut storage, contact_uuid).unwrap());

                Transaction::store_transfer(&mut storage, &mut transaction, &mut transfer);
            } else {
                Transaction::store_transaction(&mut storage, transaction);
            }
        } else {
            // Help mode
            println!("How to use: bmoney transactions update [id] [description|value|account|contact|deadline|paid|tags|observations] [value]");
            println!("Or with interactive mode: bmoney transactions update -i");
            println!("Or for pay mode: bmoney transactions update [id] pay [\"\"|YYYY-MM-DD|unpaid|today](optional) [new_value](optional)");
        }
    }

    // Remove a existing transaction
    pub fn rm(mut storage: Storage, params: Vec<String>) {

        if params.len() == 1 {
            // Shell mode

            Transaction::remove_transaction(&mut storage, params[0].trim().to_string());
        } else {
            // Help mode
            println!("How to use: bmoney transactions rm [id]");
        }
    }

    // Interface to import ofx files
    pub fn ofx(mut storage: Storage, params: Vec<String>) {

        if params.len() == 2 {
            // Shell mode

            let account = Account::get_account(&mut storage, params[0].to_owned()).unwrap();

            let mut contacts: Vec<(String, String)> = vec![];
            for co in Contact::get_contacts(&mut storage) {
                contacts.push((co.uuid, co.name));
            }
            // For transfers
            for account in Account::get_accounts(&mut storage) {
                contacts.push((account.uuid, account.name + &"(account)".to_owned()));
            }

            let mut tags_ops: Vec<(String, String)> = vec![];
            for tag in Tag::get_tags(&mut storage) {
                tags_ops.push((tag.uuid, tag.name));
            }

            let ofx = Ofx::new(params[1].to_owned())
                .expect("Couldn't open the ofx file");

            let transactions = ofx.get_transactions();

            for (i, ofx_tr) in transactions.iter().enumerate() {
                println!("Transaction {}/{}", i + 1, transactions.len());
                println!("{} on {}, memo: {}", account.format_value(ofx_tr.amount), ofx_tr.posted_at.unwrap(), ofx_tr.memo);

                let mut tr = ofx_tr.clone().build_transaction(&mut storage, account.clone());

                let mut question = "Add(y) or skip(n)?".to_string();

                if !tr.uuid.is_empty() {
                    println!("Already added in this account with description \"{}\" in {}", tr.description, tr.created_at.unwrap());
                    question = "Update(y) or skip(n)?".to_string();
                }

                if Input::read(question, false, None) != "y" {
                    continue;
                }

                tr.account = Some(account.clone());

                tr.description = Input::read("Transaction description".to_string(), true, Some(tr.description));


                let mut current_contact: Option<String> = None;

                if let Some(contact) = tr.contact {
                    current_contact = Some(contact.uuid);
                }

                let contact_uuid = Input::read_option("Contact or other account(for transfer)".to_string(), true, current_contact, contacts.clone());

                tr.contact = match Contact::get_contact(&mut storage, contact_uuid.clone()) {
                    Ok(con) => Some(con),
                    Err(_)  => None
                };


                let current_tags: Vec<String> = tr.tags.clone()
                    .iter()
                    .map(|tag| tag.uuid.clone())
                    .collect();

                tr.tags = Input::read_options("Tags".to_string(), false, current_tags, tags_ops.clone())
                    .iter()
                    .map(
                        |tag| Tag::get_tag(&mut storage, tag.to_string())
                                    .expect("Tag not found")
                    )
                    .collect();


                tr.observations = Input::read("Observations".to_string(), false, Some(tr.observations));

                Transaction::make_transaction_or_transfer(&mut storage, &mut tr, contact_uuid);

            }

        } else {
            // Help mode
            println!("How to use: bmoney transactions ofx [account id] /path/to/file.ofx");
        }
    }

    // Interface to merge two transactions
    pub fn merge(mut storage: Storage, params: Vec<String>) {

        if params.len() == 2 {
            // Shell mode

            let principal = Transaction::get_transaction(&mut storage, params[0].to_string())
                .expect("Principal transaction not found");

            let mut secondary = Transaction::get_transaction(&mut storage, params[1].to_string())
                .expect("Secondary transaction not found");

            secondary.merged_in = principal.uuid;

            Transaction::store_transaction(&mut storage, secondary);

        } else {
            // Help mode
            println!("How to use: bmoney transactions merge [principal transaction id] [secondary transaction id]");
        }
    }
}
