///
/// Blitz Money
///
/// Main of the application
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

#[macro_use]
extern crate json;
extern crate uuid;

mod ui;
mod backend;

use ui::accounts::AccountsUI;
use backend::storage::Storage;

fn main() {

    let storage = Storage { path_str: "/tmp/bmoneytmp.bms".to_string(), file: None, lines: Vec::new() };

    AccountsUI::list(storage);
}
