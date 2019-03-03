///
/// Blitz Money
///
/// Backend of module for import csv files
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use std::fs::File;
use csv::{ReaderBuilder,Reader};
use chrono::NaiveDate;
use backend::import::*;

pub struct ImportCsv {
    pub file_csv: Box<Reader<File>>,
}

impl ImportCsv {

    // Create the object for import csv file
    pub fn new(file_path: String, delimiter: String) -> Result<ImportCsv, &'static str>  {

        match ReaderBuilder::new()
                .delimiter(delimiter.as_bytes()[0])
                .from_path(file_path) {
            Ok(rdr) => Ok(ImportCsv { file_csv: Box::new(rdr) }),
            Err(_e)  => Err("Something went wrong reading the OFX file")
        }

    }

    // Get all transactions of CSV file
    pub fn get_transactions(&mut self, pos_posted: usize, pos_amount: usize, pos_memo: usize, invert_values: bool) -> Vec<Transaction> {
        let mut transactions: Vec<Transaction> = vec![];

        for result in self.file_csv.records() {
            if let Ok(row) = result {

                let sposted = row.get(pos_posted)
                                .expect("Can't find the Posted At column");
                let dtposted = NaiveDate::parse_from_str(sposted, "%Y-%m-%d")
                                .expect("Cant't parse the posted at date");

                let samount = row.get(pos_amount)
                                .expect("Can't find the amount column");
                let mut amount = samount.replace(",", ".").parse::<f32>()
                                .expect("Can't parse the amount value");

                if invert_values {
                    if amount >= 0.0 {
                        amount = 0.0 - amount;
                    } else {
                        amount = amount.abs();
                    }
                }

                let memo = row.get(pos_memo)
                                .expect("Can't find the memo column");

                transactions.push(Transaction {
                    posted_at: Some(dtposted),
                    amount: amount,
                    fitid: format!("{}-{}", sposted, memo.to_string()),
                    memo: memo.to_string()
                });
            }
        }

        transactions
    }
}
