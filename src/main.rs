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
use ui::contacts::ContactsUI;
use ui::movimentations::MovimentationsUI;
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
        } else if args[2] == "add" {
            AccountsUI::add(storage, args[3..].to_vec());
        } else if args[2] == "update" {
            AccountsUI::update(storage, args[3..].to_vec());
        } else if args[2] == "rm" {
            AccountsUI::rm(storage, args[3..].to_vec());
        } else {
            println!("How to use: bmoney accounts [list|add|update|rm]");
        }
    } else if args[1] == "contacts" {
        if args[2] == "list" {
            ContactsUI::list(storage);
        } else if args[2] == "add" {
            ContactsUI::add(storage, args[3..].to_vec());
        } else if args[2] == "update" {
            ContactsUI::update(storage, args[3..].to_vec());
        } else if args[2] == "rm" {
            ContactsUI::rm(storage, args[3..].to_vec());
        } else {
            println!("How to use: bmoney contacts [list|add|update|rm]");
        }
    } else if args[1] == "movimentations" {
        if args[2] == "list" {
            MovimentationsUI::list(storage);
        } else if args[2] == "add" {
            MovimentationsUI::add(storage, args[3..].to_vec());
        } else if args[2] == "update" {
            MovimentationsUI::update(storage, args[3..].to_vec());
        } else if args[2] == "rm" {
            MovimentationsUI::rm(storage, args[3..].to_vec());
        } else {
            println!("How to use: bmoney movimentations [list|add|update|rm]");
        }
    } else {
     println!("How to use: bmoney [module] [action]");
    }

}
