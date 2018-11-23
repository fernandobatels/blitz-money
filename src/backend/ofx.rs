///
/// Blitz Money
///
/// Backend of module for import ofx files
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use std::path::Path;
use backend::storage::*;
use backend::accounts::*;

pub struct Ofx<'a> {
    storage: &'a mut Storage,
    account: &'a mut Account,
    file_path: String,
}

impl<'a> Ofx<'a> {

    // Create the object for import ofx file
    pub fn new(storage: &'a mut Storage, account: &'a mut Account, file_path: String) -> Result<Ofx<'a>, &'static str> {

        if Path::new(&file_path).exists() {
            return Ok(Ofx { storage: storage, account: account, file_path: file_path});
        }

        Err("OFX file not found")
    }

}
