///
/// Blitz Money
///
/// Frontend/Ui of module for manange accounts of user
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use colored::*;
use prettytable::Table;

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
}
