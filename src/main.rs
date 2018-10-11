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
#[macro_use]
extern crate prettytable;
extern crate colored;
extern crate chrono;

mod ui;
mod backend;

use std::env;
use ui::accounts::AccountsUI;
use backend::storage::Storage;

fn main() {

    let storage = Storage { path_str: "/tmp/bmoneytmp.bms".to_string(), file: None, lines: Vec::new() };

    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // Without module and action
        args.push("[module]".to_string());
    } else if args.len() == 2 {
        // Without  action
        args.push("[action]".to_string());
    }

    if args[1] == "accounts" {
        if args[2] == "list" {
            AccountsUI::list(storage);
        } else {
            println!("How to use: bmoney accounts [action]");
        }
    } else {
     println!("How to use: bmoney [module] [action]");
    }

}
