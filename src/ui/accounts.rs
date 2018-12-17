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

pub struct Accounts {}

impl Accounts {

    // List of user accounts
    pub fn list(mut storage: Storage, _params: Vec<String>, is_csv: bool) {

        let accounts = Account::get_accounts(&mut storage);
        let mut table = Output::new_table();

        table.set_titles(row![b->"Name", b->"Bank", b->"Opening Balance", b->"Opening Balance Date", b->"#id"]);

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
                    .expect("Unable to set opening balance of account");
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

            let (_, totals) = Transaction::get_transactions(&mut storage, account.clone(), from, to, StatusFilter::ALL, None, None);

            let mut cols: Vec<Cell>;

            if first {
                // On first account we set the header
                // adding the news columns

                cols = vec![
                    Cell::new("Account")
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

            let name = Input::param("Account name".to_string(), true, params.clone(), 0);
            let bank = Input::param("Bank".to_string(), true, params.clone(), 1);
            let obd = Input::param_date("Opening Balance Date".to_string(), true, params.clone(), 2);
            let currency = Input::param("Currency(eg: $, R$...)".to_string(), true, params.clone(), 4);
            let ob = Input::param_money("Opening Balance".to_string(), true, params.clone(), 3);

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

            let name = Input::read("Account name".to_string(), true, None);
            let bank = Input::read("Bank".to_string(), true, None);
            let obd = Input::read_date("Opening Balance Date".to_string(), true, None);
            let currency = Input::read("Currency(eg: $, R$...)".to_string(), true, None);
            let ob = Input::read_money("Opening Balance".to_string(), true, None, currency.clone());

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
            println!("How to use: bmoney accounts add [name] [bank] [opening balance date] [opening balance] [currency]");
            println!("Or with interactive mode: bmoney accounts add -i")
        }
    }

    // Update a existing account
    pub fn update(mut storage: Storage, params: Vec<String>) {

        if params.len() == 3 {
            // Shell mode

            let mut account = Account::get_account(&mut storage, params[0].trim().to_string())
                .expect("Account not found");

            if params[1] == "name" {
                account.name = Input::param("Account name".to_string(), true, params.clone(), 2);
            } else if params[1] == "bank" {
                account.bank = Input::param("Bank".to_string(), true, params.clone(), 2);
            } else if params[1] == "obd" {
                account.open_balance_date = Input::param_date("Opening Balance Date".to_string(), true, params.clone(), 2);
            } else if params[1] == "ob" {
                account.open_balance = Input::param_money("Opening Balance".to_string(), true, params.clone(), 2);
            } else if params[1] == "currency" {
                account.currency = Input::param("Currency(eg: $, R$...)".to_string(), true, params.clone(), 2);
            } else {
                panic!("Field not found!");
            }

            Account::store_account(&mut storage, account);

        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let id = Input::read("Account id".to_string(), true, None);

            let mut account = Account::get_account(&mut storage, id)
                .expect("Account not found");

            account.name = Input::read("Account name".to_string(), true, Some(account.name));
            account.bank = Input::read("Bank".to_string(), true, Some(account.bank));
            account.open_balance_date = Input::read_date("Opening Balance Date".to_string(), true, account.open_balance_date);
            account.currency = Input::read("Currency(eg: $, R$...)".to_string(), true, Some(account.currency));
            account.open_balance = Input::read_money("Opening Balance".to_string(), true, Some(account.open_balance), account.currency.clone());

            Account::store_account(&mut storage, account);

        } else {
            // Help mode
            println!("How to use: bmoney accounts update [id] [name|bank|obd|ob|curency] [value]");
            println!("Or with interactive mode: bmoney accounts update -i")
        }
    }

    // Remove a existing account
    pub fn rm(mut storage: Storage, params: Vec<String>) {

        if params.len() == 1 {
            // Shell mode

            Account::remove_account(&mut storage, params[0].trim().to_string());

        } else {
            // Help mode
            println!("How to use: bmoney accounts rm [id]");
        }
    }
}
