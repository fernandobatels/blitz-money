///
/// Blitz Money
///
/// Frontend/Ui of module for manange accounts of user
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use colored::*;
use prettytable::Table;
use std::io;

use backend::accounts::Account;
use backend::storage::Storage;

pub struct AccountsUI {}

impl AccountsUI {

    // List of user accounts
    pub fn list(mut storage: Storage) {

        let accounts = Account::get_accounts(&mut storage);
        let mut table = Table::new();

        table.add_row(row!["Name".bold(), "Bank".bold(), "Opening Balance".bold(), "Opening Balance Date".bold(), "#id".bold()]);

        for account in accounts {

            let obcolor = match account.open_balance >= 0.0 {
                true => "green",
                false => "red"
            };

            table.add_row(row![
                account.name,
                account.bank,
                account.open_balance_formmated().color(obcolor),
                account.open_balance_date,
                account.uuid
            ]);
        }

        table.printstd();
    }

    // Create new account
    pub fn add(mut storage: Storage, params: Vec<String>) {

        if params.len() == 5 {
            // Shell mode

            Account::store_account(&mut storage, Account {
                uuid: "".to_string(),
                bank: params[1].trim().to_string(),
                name: params[0].trim().to_string(),
                open_balance: params[3].trim().parse::<f32>().unwrap(),
                open_balance_date: params[2].trim().to_string(),
                currency: params[4].trim().to_string()
            });
        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            println!("Account name:");
            let mut name = String::new();
            io::stdin().read_line(&mut name)
                .expect("Failed to read name");

            println!("Bank:");
            let mut bank = String::new();
            io::stdin().read_line(&mut bank)
                .expect("Failed to read bank");

            println!("Opening Balance Date(format YYYY-MM-DD):");
            let mut obd = String::new();
            io::stdin().read_line(&mut obd)
                .expect("Failed to read opening balance date");

            println!("Opening Balance:");
            let mut ob = String::new();
            io::stdin().read_line(&mut ob)
                .expect("Failed to read opening balance");

            println!("Currency(eg: $, R$...):");
            let mut currency = String::new();
            io::stdin().read_line(&mut currency)
                .expect("Failed to read currency");

            Account::store_account(&mut storage, Account {
                uuid: "".to_string(),
                bank: bank.trim().to_string(),
                name: name.trim().to_string(),
                open_balance: ob.trim().parse::<f32>().unwrap(),
                open_balance_date: obd.trim().to_string(),
                currency: currency.trim().to_string()
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
                account.name = params[2].trim().to_string();
            } else if params[1] == "bank" {
                account.bank = params[2].trim().to_string();
            } else if params[1] == "obd" {
                account.open_balance_date = params[2].trim().to_string();
            } else if params[1] == "ob" {
                account.open_balance = params[2].trim().parse::<f32>().unwrap();
            } else if params[1] == "currency" {
                account.currency = params[2].trim().to_string();
            } else {
                panic!("Field not found!");
            }

            Account::store_account(&mut storage, account);

        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            println!("Account id:");
            let mut id = String::new();
            io::stdin().read_line(&mut id)
                .expect("Failed to read id");

            let mut account = Account::get_account(&mut storage, id.trim().to_string())
                .expect("Account not found");

            println!("Account name: {}(keep blank for skip)", account.name);
            let mut name = String::new();
            io::stdin().read_line(&mut name)
                .expect("Failed to read name");
            if !name.trim().is_empty() {
                account.name = name.trim().to_string();
            }

            println!("Bank: {}(keep blank for skip)", account.bank);
            let mut bank = String::new();
            io::stdin().read_line(&mut bank)
                .expect("Failed to read bank");
            if !bank.trim().is_empty() {
                account.bank = bank.trim().to_string();
            }

            println!("Opening Balance Date(format YYYY-MM-DD): {}(keep blank for skip)", account.open_balance_date);
            let mut obd = String::new();
            io::stdin().read_line(&mut obd)
                .expect("Failed to read opening balance date");
            if !obd.trim().is_empty() {
                account.open_balance_date = obd.trim().to_string();
            }

            println!("Opening Balance: {}(keep blank for skip)", account.open_balance);
            let mut ob = String::new();
            io::stdin().read_line(&mut ob)
                .expect("Failed to read opening balance");
            if !ob.trim().is_empty() {
                account.open_balance = ob.trim().parse::<f32>().unwrap();
            }

            println!("Currency(eg: $, R$...): {}(keep blank for skip)", account.currency);
            let mut currency = String::new();
            io::stdin().read_line(&mut currency)
                .expect("Failed to read currency");
            if !currency.trim().is_empty() {
                account.currency = currency.trim().to_string();
            }

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
