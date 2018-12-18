///
/// Blitz Money
///
/// Frontend/Ui of module for manange money transactions
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use chrono::{Local, prelude::Datelike, NaiveDate, Duration};

use backend::transactions::Transaction;
use backend::transactions::StatusFilter;
use backend::accounts::Account;
use backend::contacts::Contact;
use backend::tags::Tag;
use backend::storage::Storage;
use backend::ofx::Ofx;
use ui::ui::*;
use i18n::*;

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

            let mut balance = totals[4].value.clone(); // Previous Balance
            let mut expected_balance = totals[4].value.clone(); // Previous Balance

            let mut table = Output::new_table();

            if show_all {
                table.set_titles(row![b->I18n::text("transactions_deadline"), b->I18n::text("transactions_description"), b->I18n::text("transactions_type"), b->I18n::text("transactions_value"), b->I18n::text("transactions_excpected_balance"), b->I18n::text("transactions_paidin"), b->I18n::text("transactions_balance"), b->I18n::text("transactions_contact"), b->I18n::text("transactions_tags"), b->"#id", b->I18n::text("transactions_byofx"), b->I18n::text("transactions_observation"), b->I18n::text("transactions_createdat"), b->I18n::text("transactions_lastupdate")]);
            } else {
                table.set_titles(row![b->I18n::text("transactions_deadline"), b->I18n::text("transactions_description"), b->I18n::text("transactions_type"), b->I18n::text("transactions_value"), b->I18n::text("transactions_excpected_balance"), b->I18n::text("transactions_paidin"), b->I18n::text("transactions_balance"), b->I18n::text("transactions_contact"), b->I18n::text("transactions_tags"), b->"#id", b->I18n::text("transactions_byofx")]);
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

                if transaction.paid_in.is_some() {
                    balance += transaction.value;
                }
                expected_balance += transaction.value;

                let mut row = table.add_row(row![
                    transaction.deadline.unwrap(),
                    transaction.description,
                    "C",
                    Fg->transaction.value_formmated(),
                    Fg->account.format_value(expected_balance),
                    transaction.paid_in_formmated(),
                    Fg->account.format_value(balance),
                    "",
                    tags.join(", "),
                    transaction.clone().id(),
                    by_ofx
                ]);

                if transaction.value < 0.0 {
                    row.set_cell(cell!("D"), 2)
                        .expect(&I18n::text("transactions_unable_to_set_d"));
                    row.set_cell(cell!(Fr->transaction.value_formmated()), 3)
                        .expect(&I18n::text("transactions_unable_to_set_value"));
                }

                if expected_balance < 0.0 {
                    row.set_cell(cell!(Fr->account.format_value(expected_balance)), 4)
                        .expect(&I18n::text("transactions_unable_to_set_expected_balance"));
                }

                if balance < 0.0 {
                    row.set_cell(cell!(Fr->account.format_value(balance)), 6)
                        .expect(&I18n::text("transactions_unable_to_set_balance"));
                }

                if transaction.transfer.is_some() {
                    row.set_cell(cell!("T"), 2)
                        .expect(&I18n::text("transactions_unable_to_set_t"));

                    // In transcations we show the destination account on place of contact
                    row.set_cell(cell!(transaction.transfer.unwrap().account.clone().unwrap().name + &I18n::text("transactions_caccount")), 7)
                        .expect(&I18n::text("transactions_unable_to_set_account"));
                } else {
                    row.set_cell(cell!(transaction.contact.clone().unwrap().name), 7)
                        .expect(&I18n::text("transactions_unable_to_set_contact"));
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
                table.add_row(row!["", "", "", "", "", "", "", "", "", "", "", "", "", ""]);
            } else {
                table.add_row(row!["", "", "", "", "", "", "", "", "", "", ""]);
            }

            for total in totals {

                let mut row = table.add_row(row![
                    "",
                    b->total.label,
                    "",
                    Fg->account.format_value(total.value),
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    ""
                ]);

                if total.value < 0.0 {
                    row.set_cell(cell!(Fr->account.format_value(total.value)), 3)
                        .expect(&I18n::text("transactions_unable_to_set_total"));
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
            println!("{}", I18n::text("transactions_how_to_use_list"));
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
            let mut repetitions;
            let mut repetitions_interval;

            if params.len() >= 5 {
                // Shell mode

                description = Input::param(I18n::text("transactions_description"), true, params.clone(), 0);

                let account_uuid = Input::param(I18n::text("transactions_account"), true, params.clone(), 2);
                account = Some(Account::get_account(&mut storage, account_uuid).unwrap());

                value = Input::param_money(I18n::text("transactions_lvalue"), true, params.clone(), 1);

                contact_uuid = Input::param(I18n::text("transactions_contact"), true, params.clone(), 3);

                deadline = Input::param_date(I18n::text("transactions_deadline"), true, params.clone(), 4);
                paid_in = Input::param_date(I18n::text("transactions_paidin"), false, params.clone(), 5);

                let tags_str = Input::param(I18n::text("transactions_tags"), false, params.clone(), 6);

                if !tags_str.is_empty() {
                    for tag in tags_str.split(",") {
                        tags.push(
                            Tag::get_tag(&mut storage, tag.to_string())
                                .expect(&I18n::text("tags_not_found"))
                        );
                    }
                }

                observations = Input::param(I18n::text("transactions_observations"), false, params.clone(), 7);

                repetitions = Input::param_int(I18n::text("transactions_repetitions"), false, params.clone(), 8);
                repetitions_interval = Input::param_int(I18n::text("transactions_repetitions_interval"), false, params.clone(), 9);

            } else {
                // Interactive mode

                description = Input::read(I18n::text("transactions_description"), true, None);

                let mut accounts: Vec<(String, String)> = vec![];
                for ac in Account::get_accounts(&mut storage) {
                    accounts.push((ac.uuid, ac.name));
                }

                let account_uuid = Input::read_option(I18n::text("transactions_account"), true, None, accounts.clone());
                account = Some(Account::get_account(&mut storage, account_uuid).unwrap());

                value = Input::read_money(I18n::text("transactions_lvalue"), true, None, account.clone().unwrap().currency);

                let mut contacts: Vec<(String, String)> = vec![];
                for co in Contact::get_contacts(&mut storage) {
                    contacts.push((co.uuid, co.name));
                }
                // For transfers
                for (uuid, name) in accounts {
                    contacts.push((uuid, name + &I18n::text("transactions_caccount")));
                }

                contact_uuid = Input::read_option(I18n::text("transactions_contact_or_other_account"), true, None, contacts);

                deadline = Input::read_date(I18n::text("transactions_deadline"), true, None);
                paid_in = Input::read_date(I18n::text("transactions_paidin"), false, None);

                let mut tags_ops: Vec<(String, String)> = vec![];
                for tag in Tag::get_tags(&mut storage) {
                    tags_ops.push((tag.uuid, tag.name));
                }

                tags = Input::read_options(I18n::text("transactions_tags"), false, vec![], tags_ops)
                    .iter()
                    .map(
                        |tag| Tag::get_tag(&mut storage, tag.to_string())
                                    .expect(&I18n::text("tags_not_found"))
                    )
                    .collect();

                observations = Input::read(I18n::text("transactions_observations"), false, None);

                repetitions = Input::read_int(I18n::text("transactions_repetitions"), false, None);
                if repetitions > 0 {
                    // We only ask the interval if the user input the repetitions...
                    repetitions_interval = Input::read_int(I18n::text("transactions_repetitions_interval"), false, None);
                } else {
                    repetitions_interval = 0;
                }
            }

            let contact = match Contact::get_contact(&mut storage, contact_uuid.clone()) {
                Ok(con) => Some(con),
                Err(_)  => None
            };

            let mut mov_template = Transaction {
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

            if repetitions <= 0 {
                repetitions = 1;
            }

            if repetitions_interval <= 0 {
                repetitions_interval = 1;
            }

            for rep in 0..repetitions {

                let mut mov = mov_template.clone();

                if repetitions > 1 {
                    mov.description.push_str(&format!(" [{}/{}]", rep + 1, repetitions));
                }

                let tr_uuid = Transaction::make_transaction_or_transfer(&mut storage, &mut mov, contact_uuid.clone());

                let duration = Duration::days(repetitions_interval.into());

                mov_template.deadline = Some(mov_template.deadline.unwrap() + duration);

                if mov_template.paid_in.is_some() {
                    mov_template.paid_in = Some(mov_template.paid_in.unwrap() + duration);
                }

                mov_template.previous_repetition = tr_uuid;
            }

        } else {
            // Help mode
            println!("{}", I18n::text("transactions_how_to_use_add"));
        }
    }

    // Update a existing transaction
    pub fn update(mut storage: Storage, params: Vec<String>) {

        if params.len() >= 2 && params[1] == "pay" {
            // Pay mode

            let mut transaction = Transaction::get_transaction(&mut storage, params[0].trim().to_string())
                .expect(&I18n::text("transactions_not_found"));

            // Update the paid date
            if params.len() >= 3 && params[2] != "today" {
                if params[2].is_empty() || params[2] == "unpaid" {
                    transaction.paid_in = None;
                } else {
                    // Is a date value
                    transaction.paid_in = Input::param_date(I18n::text("transactions_paidin"), false, params.clone(), 2);
                }
            } else {
                // No date, no empty or the 'today' value we set this transaction as paid today
                transaction.paid_in = Some(Local::today().naive_local());
            }

            // Update the value
            if params.len() >= 4 {
                transaction.value = Input::param_money(I18n::text("transactions_lvalue"), true, params.clone(), 3);
            }

            if transaction.transfer.is_some() {
                Transaction::store_transfer(&mut storage, &mut transaction.clone(), &mut transaction.transfer.unwrap());
            } else {
                Transaction::store_transaction(&mut storage, transaction);
            }

        } else if params.len() == 3 {
            // Shell mode

            let mut transaction = Transaction::get_transaction(&mut storage, params[0].trim().to_string())
                .expect(&I18n::text("transactions_not_found"));

            if params[1] == "description" {
                transaction.description = Input::param(I18n::text("transactions_description"), true, params.clone(), 2);
            } else if params[1] == "value" {
                transaction.value = Input::param_money(I18n::text("transactions_lvalue"), true, params.clone(), 2);
            } else if params[1] == "account" {
                let account_uuid = Input::param(I18n::text("transactions_account"), true, params.clone(), 2);
                transaction.account = Some(Account::get_account(&mut storage, account_uuid).unwrap());
            } else if params[1] == "contact" {
                let contact_uuid = Input::param(I18n::text("transactions_contact"), true, params.clone(), 2);
                if transaction.transfer.is_some() {
                    transaction.account = Some(Account::get_account(&mut storage, contact_uuid).unwrap());
                } else {
                    transaction.contact = Some(Contact::get_contact(&mut storage, contact_uuid).unwrap());
                }
            } else if params[1] == "deadline" {
                transaction.deadline = Input::param_date(I18n::text("transactions_deadline"), true, params.clone(), 2);
            } else if params[1] == "paid_in" {
                transaction.paid_in = Input::param_date(I18n::text("transactions_paidin"), false, params.clone(), 2);
            } else if params[1] == "tags" {
                let tags_str = Input::param(I18n::text("transactions_tags"), false, params.clone(), 2);
                transaction.tags = vec![];

                if !tags_str.is_empty() {
                    for tag in tags_str.split(",") {
                        transaction.tags.push(
                            Tag::get_tag(&mut storage, tag.to_string())
                                .expect(&I18n::text("tags_not_found"))
                        );
                    }
                }
            } else if params[1] == "observations" {
                transaction.observations = Input::param(I18n::text("transactions_observations"), false, params.clone(), 2);
            } else {
                panic!(I18n::text("field_not_found"));
            }

            if transaction.transfer.is_some() {
                Transaction::store_transfer(&mut storage, &mut transaction.clone(), &mut transaction.transfer.unwrap());
            } else {
                Transaction::store_transaction(&mut storage, transaction);
            }

        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let id = Input::read("#id".to_string(), true, None);

            let mut transaction = Transaction::get_transaction(&mut storage, id)
                .expect(&I18n::text("transactions_not_found"));

            transaction.description = Input::read(I18n::text("transactions_description"), true, Some(transaction.description));

            let mut accounts: Vec<(String, String)> = vec![];
            for ac in Account::get_accounts(&mut storage) {
                accounts.push((ac.uuid, ac.name));
            }

            let account_uuid = Input::read_option(I18n::text("transactions_account"), true, Some(transaction.account.clone().unwrap().uuid), accounts.clone());
            transaction.account = Some(Account::get_account(&mut storage, account_uuid).unwrap());

            transaction.value = Input::read_money(I18n::text("transactions_lvalue"), true, Some(transaction.value), transaction.account.clone().unwrap().currency);

            let contact_uuid: String;

            if transaction.transfer.is_some() {
                contact_uuid = Input::read_option(I18n::text("transactions_destination_account"),true, Some(transaction.transfer.clone().unwrap().uuid), accounts);
            } else {
                let mut contacts: Vec<(String, String)> = vec![];
                for co in Contact::get_contacts(&mut storage) {
                    contacts.push((co.uuid, co.name));
                }

                contact_uuid = Input::read_option(I18n::text("transactions_contact"), true, Some(transaction.contact.clone().unwrap().uuid), contacts);
                transaction.contact = Some(Contact::get_contact(&mut storage, contact_uuid.clone()).unwrap());
            }

            transaction.deadline = Input::read_date(I18n::text("transactions_deadline"), true, transaction.deadline);
            transaction.paid_in = Input::read_date(I18n::text("transactions_paidin"), false, transaction.paid_in);

            let mut tags_ops: Vec<(String, String)> = vec![];
            for tag in Tag::get_tags(&mut storage) {
                tags_ops.push((tag.uuid, tag.name));
            }

            let current_tags: Vec<String> = transaction.tags.clone()
                .iter()
                .map(|tag| tag.uuid.clone())
                .collect();

            transaction.tags = Input::read_options(I18n::text("transactions_tags"), false, current_tags, tags_ops)
                .iter()
                .map(
                    |tag| Tag::get_tag(&mut storage, tag.to_string())
                                .expect(&I18n::text("tags_not_found"))
                )
                .collect();

            transaction.observations = Input::read(I18n::text("transactions_observations"), false, Some(transaction.observations));

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
            println!("{}", I18n::text("transactions_how_to_use_update"));
        }
    }

    // Remove a existing transaction
    pub fn rm(mut storage: Storage, params: Vec<String>) {

        if params.len() == 1 {
            // Shell mode

            Transaction::remove_transaction(&mut storage, params[0].trim().to_string());
        } else {
            // Help mode
            println!("{}", I18n::text("transactions_how_to_use_rm"));
        }
    }

    // Interface to import ofx files
    pub fn ofx(mut storage: Storage, mut params: Vec<String>) {

        if params.len() >= 2 {
            // Shell mode

            let account = Account::get_account(&mut storage, params[0].to_owned()).unwrap();

            let mut auto_skip = false;
            if Input::extract_param(&mut params, "--auto-skip".to_string()) {
                auto_skip = true;
            }


            let mut contacts: Vec<(String, String)> = vec![];
            for co in Contact::get_contacts(&mut storage) {
                contacts.push((co.uuid, co.name));
            }
            // For transfers
            for account in Account::get_accounts(&mut storage) {
                contacts.push((account.uuid, account.name + &I18n::text("transactions_caccount")));
            }

            let mut tags_ops: Vec<(String, String)> = vec![];
            for tag in Tag::get_tags(&mut storage) {
                tags_ops.push((tag.uuid, tag.name));
            }

            let ofx = Ofx::new(params[1].to_owned())
                .expect(&I18n::text("transactions_couldnt_open_ofx"));

            let transactions = ofx.get_transactions();

            for (i, ofx_tr) in transactions.iter().enumerate() {
                println!("{} {}/{}", I18n::text("transactions_ofx_index"), i + 1, transactions.len());
                println!("{} {} {}, memo: {}", I18n::text("transactions_ofx_on"), account.format_value(ofx_tr.amount), ofx_tr.posted_at.unwrap(), ofx_tr.memo);

                let mut tr = ofx_tr.clone().build_transaction(&mut storage, account.clone());

                let mut question = I18n::text("transactions_ofx_add_skip");

                if !tr.uuid.is_empty() {
                    println!("{} \"{}\" in {}", I18n::text("transactions_ofx_already"), tr.description, tr.created_at.unwrap());
                    question = I18n::text("transactions_ofx_update_skip");

                    if auto_skip {
                        println!("{}", I18n::text("transactions_ofx_auto_skip"));
                        continue;
                    }
                }

                if Input::read(question, false, None) != "y" {
                    continue;
                }

                tr.account = Some(account.clone());

                tr.description = Input::read(I18n::text("transactions_description"), true, Some(tr.description));


                let mut current_contact: Option<String> = None;

                if let Some(contact) = tr.contact {
                    current_contact = Some(contact.uuid);
                }

                let contact_uuid = Input::read_option(I18n::text("transactions_contact_or_other_account"), true, current_contact, contacts.clone());

                tr.contact = match Contact::get_contact(&mut storage, contact_uuid.clone()) {
                    Ok(con) => Some(con),
                    Err(_)  => None
                };


                let current_tags: Vec<String> = tr.tags.clone()
                    .iter()
                    .map(|tag| tag.uuid.clone())
                    .collect();

                tr.tags = Input::read_options(I18n::text("transactions_tags"), false, current_tags, tags_ops.clone())
                    .iter()
                    .map(
                        |tag| Tag::get_tag(&mut storage, tag.to_string())
                                    .expect(&I18n::text("tags_not_found"))
                    )
                    .collect();


                tr.observations = Input::read(I18n::text("transactions_observations"), false, Some(tr.observations));

                Transaction::make_transaction_or_transfer(&mut storage, &mut tr, contact_uuid);

            }

        } else {
            // Help mode
            println!("{}", I18n::text("transactions_how_to_use_ofx"));
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

            if secondary.transfer.is_some() {
                Transaction::store_transfer(&mut storage, &mut secondary.clone(), &mut secondary.transfer.unwrap());
            } else {
                Transaction::store_transaction(&mut storage, secondary);
            }

        } else {
            // Help mode
            println!("{}", I18n::text("transactions_how_to_use_merge"));
        }
    }
}
