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
extern crate chrono;
extern crate csv;

mod ui;
mod backend;

use std::env;
use ui::accounts::Accounts;
use ui::contacts::Contacts;
use ui::movimentations::Movimentations;
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

    let mut is_csv = false;

    for (i, param) in args.clone().iter().enumerate() {
        if param == "--use-csv" {
            is_csv = true;
            args.remove(i);
            break;
        }
    }

    if args[1] == "accounts" {
        if args[2] == "list" {
            Accounts::list(storage, args[3..].to_vec(), is_csv);
        } else if args[2] == "add" {
            Accounts::add(storage, args[3..].to_vec());
        } else if args[2] == "update" {
            Accounts::update(storage, args[3..].to_vec());
        } else if args[2] == "rm" {
            Accounts::rm(storage, args[3..].to_vec());
        } else {
            println!("How to use: bmoney accounts [list|add|update|rm]");
        }
    } else if args[1] == "contacts" {
        if args[2] == "list" {
            Contacts::list(storage, args[3..].to_vec(), is_csv);
        } else if args[2] == "add" {
            Contacts::add(storage, args[3..].to_vec());
        } else if args[2] == "update" {
            Contacts::update(storage, args[3..].to_vec());
        } else if args[2] == "rm" {
            Contacts::rm(storage, args[3..].to_vec());
        } else {
            println!("How to use: bmoney contacts [list|add|update|rm]");
        }
    } else if args[1] == "movimentations" {
        if args[2] == "list" {
            Movimentations::list(storage, args[3..].to_vec(), is_csv);
        } else if args[2] == "add" {
            Movimentations::add(storage, args[3..].to_vec());
        } else if args[2] == "update" {
            Movimentations::update(storage, args[3..].to_vec());
        } else if args[2] == "rm" {
            Movimentations::rm(storage, args[3..].to_vec());
        } else {
            println!("How to use: bmoney movimentations [list|add|update|rm]");
        }
    } else {
     println!("How to use: bmoney [module] [action]");
    }

}
