///
/// Blitz Money
///
/// Frontend/Ui of module for manange accounts of user
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use backend::accounts::Account;
use backend::storage::Storage;

pub struct AccountsUI {
}

impl AccountsUI {

    pub fn list(storage: Storage) {

        let accounts = Account::get_accounts(storage);

        println!("User accounts");

        for account in accounts {
            println!("{:?}", account);
        }

    }
}
