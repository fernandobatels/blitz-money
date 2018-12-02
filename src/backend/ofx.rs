///
/// Blitz Money
///
/// Backend of module for import ofx files
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use std::fs::File;
use std::io::prelude::*;
use xmltree::*;
use chrono::{Local, DateTime, NaiveDate};
use backend::storage::*;
use backend::accounts::*;

pub struct Ofx<'a> {
    storage: &'a mut Storage,
    account: &'a mut Account,
    pub file_doc: Box<Element>,
}

#[derive(Debug)]
pub struct Transaction {
    pub posted_at: Option<NaiveDate>,
    pub amount: f32,
    pub fitid: String, // Financial instituion id
    pub memo: String
}

impl<'a> Ofx<'a> {

    // Create the object for import ofx file
    pub fn new(storage: &'a mut Storage, account: &'a mut Account, file_path: String) -> Result<Ofx<'a>, &'static str> {

        let file = File::open(file_path.clone());

        if file.is_err() {
            return Err("OFX file not found");
        }

        let mut file_content = String::new();
        if file.unwrap().read_to_string(&mut file_content).is_err() {
            return Err("Something went wrong reading the OFX file");
        }

        if !file_content.contains("<OFX>") {
            return Err("The OFX file not contains <OFX> tag");
        }

        let xml_split: Vec<&str> = file_content.split("<OFX>").collect();

        let xml_str = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?><OFX>{}", xml_split[1]);

        let xml = Element::parse(xml_str.as_bytes());

        if xml.is_err() {
            return Err("Invalid XML content in OFX file");
        }

        Ok(Ofx { storage: storage, account: account, file_doc: Box::new(xml.unwrap()) })
    }

    // Get all transactions of OFX file
    pub fn get_transactions(mut self) -> Vec<Transaction> {
        let mut transactions: Vec<Transaction> = vec![];

        let tran_list = self.file_doc
            .get_mut_child("BANKMSGSRSV1").expect("Can't find BANKMSGSRSV1 element")
            .get_mut_child("STMTTRNRS").expect("Can't find STMTTRNRS element")
            .get_mut_child("STMTRS").expect("Can't find STMTRS element")
            .get_mut_child("BANKTRANLIST").expect("Can't find BANKTRANLIST element");

        for tr in tran_list.clone().children {
            if tr.name == "STMTTRN".to_string() {

                let dtposted = tr.get_child("DTPOSTED")
                    .expect("Can't find DTPOSTED element")
                    .clone().text.unwrap();

                let trnamt = tr.get_child("TRNAMT")
                    .expect("Can't find TRNAMT element")
                    .clone().text.unwrap();

                let fitid = tr.get_child("FITID")
                    .expect("Can't find FITID element")
                    .clone().text.unwrap();

                let memo = tr.get_child("MEMO")
                    .expect("Can't find MEMO element")
                    .clone().text.unwrap();

                transactions.push(Transaction {
                    posted_at: Some(NaiveDate::parse_from_str(&dtposted.chars().take(8).collect::<String>(), "%Y%m%d").unwrap()),
                    amount: trnamt.parse::<f32>().unwrap(),
                    fitid: fitid,
                    memo: memo
                });
            }
        }

        return transactions;
    }
}
