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
extern crate assert_cmd;

mod ui;
mod backend;

use std::env;
use ui::tags::Tags;
use ui::accounts::Accounts;
use ui::contacts::Contacts;
use ui::movimentations::Movimentations;
use ui::ui::Input;
use backend::storage::Storage;

fn main() {

    let mut args: Vec<String> = env::args().collect();

    let path_str: String;

    if let Some(file) = Input::extract_named_param(&mut args, "--storage-file=".to_string()) {
        // When the user set a file for the storage
        path_str = file;
    } else {
        // Default file path
        let home_dir = env::home_dir()
            .expect("Impossible to get your home dir!");
        let home_dir_str = home_dir.to_str()
            .expect("Fail on get your home string!");

        path_str = home_dir_str.to_owned() + &"/.bmoney.bms".to_string();
    }

    let storage = Storage { path_str: path_str, file: None, lines: Vec::new() };

    if args.len() == 1 {
        // Without module and action
        args.push("[module]".to_string());
    } else if args.len() == 2 {
        // Without  action
        args.push("[action]".to_string());
    }

    let is_csv = Input::extract_param(&mut args, "--use-csv".to_string());

    if args[1] == "accounts" {
        if args[2] == "list" {
            Accounts::list(storage, args[3..].to_vec(), is_csv);
        } else if args[2] == "status" {
            Accounts::status(storage, args[3..].to_vec(), is_csv);
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
    } else if args[1] == "tags" {
        if args[2] == "list" {
            Tags::list(storage, args[3..].to_vec(), is_csv);
        } else if args[2] == "add" {
            Tags::add(storage, args[3..].to_vec());
        } else if args[2] == "update" {
            Tags::update(storage, args[3..].to_vec());
        } else if args[2] == "rm" {
            Tags::rm(storage, args[3..].to_vec());
        } else {
            println!("How to use: bmoney tags [list|add|update|rm]");
        }
    } else {
     println!("How to use: bmoney [module] [action]");
    }

}
