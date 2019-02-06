///
/// Blitz Money
///
/// Backend of module for support the importation of files
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use chrono::NaiveDate;
use backend::transactions;
use backend::storage::Storage;
use backend::accounts::Account;

pub struct Import {

}

#[derive(Clone)]
pub struct Transaction {
    pub posted_at: Option<NaiveDate>,
    pub amount: f32,
    pub fitid: String, // Financial instituion id
    pub memo: String,
}

impl Import {

    // Populate de index of transactions for use on
    // import files
    pub fn index(storage: &mut Storage) {

        let accounts = Account::get_accounts(storage);

        for account in accounts {
            let transactions = transactions::Transaction::get_transactions_simple(storage, account.clone());

            for transaction in transactions {
                if !transaction.ofx_fitid.is_empty() {
                    let key = format!("fitid_{}", account.clone().uuid);
                    storage.set_index("transactions".to_string(), key, transaction.ofx_fitid.clone(), transaction.uuid.clone());
                }
            }
        }
    }
}

impl Transaction {

    // Make a transaction for storage or return the old
    // transaction if the fitid exists on transactions of
    // the account
    pub fn build_transaction(self, storage: &mut Storage, account: Account) -> transactions::Transaction {

        if let Some(transaction) = self.clone().find_transaction_by_fitid(storage, account) {
            return transaction;
        }

        // This abstract file use the ofx prefix becase we
        // start this project with only support for ofx format

        transactions::Transaction {
            description: self.memo.clone(),
            value: self.amount,
            deadline: self.posted_at,
            paid_in: self.posted_at,
            ofx_memo: self.memo.clone(),
            ofx_fitid: self.fitid.clone(),
            ..Default::default()
        }
    }

    // Get, if already exists, the trasaction by fitid
    fn find_transaction_by_fitid(self, storage: &mut Storage, account: Account) -> Option<transactions::Transaction> {

        storage.start_section("transactions".to_string());

        let mut data = storage.get_section_data("transactions".to_string());

        let key = format!("fitid_{}", account.clone().uuid);

        if data.find_by_index(key, self.fitid) {
            if let Ok(transaction) = data.next::<transactions::Transaction>() {
                return Some(transaction);
            }
        }

        None
    }

}
