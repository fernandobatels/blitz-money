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
    pub fn list(storage: Storage) {

        let accounts = Account::get_accounts(storage);
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
    pub fn add(storage: Storage, params: Vec<String>) {

        if params.len() == 5 {
            // Shell mode

            Account::storage_account(storage, Account {
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

            Account::storage_account(storage, Account {
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
}
