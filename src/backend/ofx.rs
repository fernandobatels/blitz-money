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

pub struct Movimentation {
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

    // Get all movimentations of OFX file
    pub fn get_movimentations(self) -> Vec<Movimentation> {
        let mut movimentations: Vec<Movimentation> = vec![];

        return movimentations;
    }
}
