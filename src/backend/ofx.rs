///
/// Blitz Money
///
/// Backend of module for import ofx files
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use std::fs::File;
use std::io::prelude::*;
use dummy_xml::parser::*;
use backend::storage::*;
use backend::accounts::*;

pub struct Ofx<'a> {
    storage: &'a mut Storage,
    account: &'a mut Account,
    file_doc: Document<'a>,
    file_path: String,
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

        let xml_str = format!("<OFX>{}", xml_split[1]);

        let xml = parse_string(&xml_str);

        if xml.is_err() {
            return Err("Invalid XML content in OFX file");
        }

        Ok(Ofx { storage: storage, account: account, file_path: file_path, file_doc: xml.unwrap() })
    }

}
