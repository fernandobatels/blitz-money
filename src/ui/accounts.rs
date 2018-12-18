///
/// Blitz Money
///
/// Frontend/Ui of module for manange accounts of user
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use backend::accounts::Account;
use backend::storage::Storage;
use ui::ui::*;
use chrono::{Local, prelude::Datelike, NaiveDate};
use prettytable::{Row, Cell, Attr, color};
use backend::transactions::Transaction;
use backend::transactions::StatusFilter;
use i18n::*;

pub struct Accounts {}

impl Accounts {

    // List of user accounts
    pub fn list(mut storage: Storage, _params: Vec<String>, is_csv: bool) {

        let accounts = Account::get_accounts(&mut storage);
        let mut table = Output::new_table();

        table.set_titles(row![b->I18n::text("accounts_name"), b->I18n::text("accounts_bank"), b->I18n::text("accounts_ob"), b->I18n::text("accounts_obd"), b->"#id"]);

        for account in accounts {

            let mut row = table.add_row(row![
                account.name,
                account.bank,
                Fg->account.open_balance_formmated(),
                account.open_balance_date.unwrap(),
                account.clone().id()
            ]);

            if account.open_balance < 0.0 {
                row.set_cell(cell!(Fr->account.open_balance_formmated()), 2)
                    .expect(&I18n::text("accounts_unable_to_set_opening_balance_of_account"));
            }
        }

        Output::print_table(table, is_csv);
    }

    // Show status of all accounts
    pub fn status(mut storage: Storage, params: Vec<String>, is_csv: bool) {

        let mut from = Local::now().with_day(1).unwrap().date().naive_local();
        let mut to = Local::now().with_day(30).unwrap().date().naive_local();// yes, fix to get last day of month

        if params.len() == 2 {
            from = NaiveDate::parse_from_str(&params[0].trim().to_string(), "%Y-%m-%d").unwrap();
            to = NaiveDate::parse_from_str(&params[1].trim().to_string(), "%Y-%m-%d").unwrap();
        }

        let accounts = Account::get_accounts(&mut storage);
        let mut table = Output::new_table();

        let mut first = true;

        for account in accounts {

            let (_, totals) = Transaction::get_transactions(&mut storage, account.clone(), from, to, StatusFilter::ALL, None);

            let mut cols: Vec<Cell>;

            if first {
                // On first account we set the header
                // adding the news columns

                cols = vec![
                    Cell::new(&I18n::text("accounts_name"))
                        .with_style(Attr::Bold)
                ];

                for total in totals.clone() {
                    cols.push(
                        Cell::new(&total.label)
                            .with_style(Attr::Bold)
                    );
                }

                table.set_titles(Row::new(cols));

                first = false;
            }

            cols = vec![
                Cell::new(&account.name)
            ];

            for total in totals {
                let mut cell = Cell::new(&account.format_value(total.value));

                if total.value >= 0.0 {
                    cell = cell.with_style(Attr::ForegroundColor(color::GREEN));
                } else {
                    cell = cell.with_style(Attr::ForegroundColor(color::RED));
                }

                cols.push(cell);
            }

            table.add_row(Row::new(cols));
        }

        Output::print_table(table, is_csv);
    }

    // Create new account
    pub fn add(mut storage: Storage, params: Vec<String>) {

        if params.len() == 5 {
            // Shell mode

            let name = Input::param(I18n::text("accounts_name"), true, params.clone(), 0);
            let bank = Input::param(I18n::text("accounts_bank"), true, params.clone(), 1);
            let obd = Input::param_date(I18n::text("accounts_obd"), true, params.clone(), 2);
            let currency = Input::param(I18n::text("accounts_currency"), true, params.clone(), 4);
            let ob = Input::param_money(I18n::text("accounts_ob"), true, params.clone(), 3);

            Account::store_account(&mut storage, Account {
                uuid: "".to_string(),
                bank: bank,
                name: name,
                open_balance: ob,
                open_balance_date: obd,
                currency: currency
            });
        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let name = Input::read(I18n::text("accounts_name"), true, None);
            let bank = Input::read(I18n::text("accounts_bank"), true, None);
            let obd = Input::read_date(I18n::text("accounts_obd"), true, None);
            let currency = Input::read(I18n::text("accounts_currency"), true, None);
            let ob = Input::read_money(I18n::text("accounts_ob"), true, None, currency.clone());

            Account::store_account(&mut storage, Account {
                uuid: "".to_string(),
                bank: bank,
                name: name,
                open_balance: ob,
                open_balance_date: obd,
                currency: currency
            });
        } else {
            // Help mode
            println!("{}", I18n::text("accounts_how_to_use_add"));
        }
    }

    // Update a existing account
    pub fn update(mut storage: Storage, params: Vec<String>) {

        if params.len() == 3 {
            // Shell mode

            let mut account = Account::get_account(&mut storage, params[0].trim().to_string())
                .expect(&I18n::text("accounts_not_found"));

            if params[1] == "name" {
                account.name = Input::param(I18n::text("accounts_name"), true, params.clone(), 2);
            } else if params[1] == "bank" {
                account.bank = Input::param(I18n::text("accounts_bank"), true, params.clone(), 2);
            } else if params[1] == "obd" {
                account.open_balance_date = Input::param_date(I18n::text("accounts_obd"), true, params.clone(), 2);
            } else if params[1] == "ob" {
                account.open_balance = Input::param_money(I18n::text("accounts_ob"), true, params.clone(), 2);
            } else if params[1] == "currency" {
                account.currency = Input::param(I18n::text("accounts_currency"), true, params.clone(), 2);
            } else {
                panic!(I18n::text("field_not_found"));
            }

            Account::store_account(&mut storage, account);

        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let id = Input::read("#id".to_string(), true, None);

            let mut account = Account::get_account(&mut storage, id)
                .expect(&I18n::text("accounts_not_found"));

            account.name = Input::read(I18n::text("accounts_name"), true, Some(account.name));
            account.bank = Input::read(I18n::text("accounts_bank"), true, Some(account.bank));
            account.open_balance_date = Input::read_date(I18n::text("accounts_obd"), true, account.open_balance_date);
            account.currency = Input::read(I18n::text("accounts_currency"), true, Some(account.currency));
            account.open_balance = Input::read_money(I18n::text("accounts_ob"), true, Some(account.open_balance), account.currency.clone());

            Account::store_account(&mut storage, account);

        } else {
            // Help mode
            println!("{}", I18n::text("accounts_how_to_use_update"));
        }
    }

    // Remove a existing account
    pub fn rm(mut storage: Storage, params: Vec<String>) {

        if params.len() == 1 {
            // Shell mode

            Account::remove_account(&mut storage, params[0].trim().to_string());

        } else {
            // Help mode
            println!("{}", I18n::text("accounts_how_to_use_rm"));
        }
    }
}
