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
extern crate xmltree;
extern crate dirs;
extern crate json_gettext;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate lazy_static_include;
extern crate icalendar;

mod ui;
mod backend;
mod i18n;

use std::env;
use std::collections::HashMap;
use std::cell::RefCell;
use backend::import::Import;
use ui::tags::Tags;
use ui::accounts::Accounts;
use ui::contacts::Contacts;
use ui::transactions::Transactions;
use ui::rules::Rules;
use ui::forecasts::Forecasts;
use ui::ui::*;
use backend::storage::Storage;
use i18n::*;

fn main() {

    let lang: String = match env::var_os("LANGUAGE") {
        Some(val) => val.into_string().unwrap(),
        None => "en_US".to_string()
    };

    I18n::config(lang);

    let mut args: Vec<String> = env::args().collect();

    let path_str: String;

    if let Some(file) = Input::extract_named_param(&mut args, "--storage-file=".to_string()) {
        // When the user set a file for the storage
        path_str = file;
    } else {
        // Default file path
        let home_dir = dirs::home_dir()
            .expect(&I18n::text("impossible_to_get_your_home_dir"));
        let home_dir_str = home_dir.to_str()
            .expect(&I18n::text("fail_on_get_your_home_string"));

        path_str = home_dir_str.to_owned() + &"/.bmoney.bms".to_string();
    }

    let mut storage = Storage { path_str: path_str, file: None, lines: Vec::new(), index: RefCell::new(HashMap::new()) };

    Import::index(&mut storage);

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
            println!("{}: bmoney accounts [list|add|update|rm|status]", I18n::text("how_to_use"));
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
            println!("{}: bmoney contacts [list|add|update|rm]", I18n::text("how_to_use"));
        }
    } else if args[1] == "transactions" {
        if args[2] == "list" {
            Transactions::list(storage, args[3..].to_vec(), is_csv);
        } else if args[2] == "add" {
            Transactions::add(storage, args[3..].to_vec());
        } else if args[2] == "update" {
            Transactions::update(storage, args[3..].to_vec());
        } else if args[2] == "rm" {
            Transactions::rm(storage, args[3..].to_vec());
        } else if args[2] == "ofx" {
            Transactions::ofx(storage, args[3..].to_vec());
        } else if args[2] == "csv" {
            Transactions::csv(storage, args[3..].to_vec());
        } else if args[2] == "merge" {
            Transactions::merge(storage, args[3..].to_vec());
        } else if args[2] == "calendar" {
            Transactions::calendar(storage, args[3..].to_vec());
        } else {
            println!("{}: bmoney transactions [list|add|update|rm|ofx|merge|calendar]", I18n::text("how_to_use"));
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
            println!("{}: bmoney tags [list|add|update|rm]", I18n::text("how_to_use"));
        }
    } else if args[1] == "rules" {
        if args[2] == "list" {
            Rules::list(storage, args[3..].to_vec(), is_csv);
        } else if args[2] == "add" {
            Rules::add(storage, args[3..].to_vec());
        } else if args[2] == "update" {
            Rules::update(storage, args[3..].to_vec());
        } else if args[2] == "rm" {
            Rules::rm(storage, args[3..].to_vec());
        } else {
            println!("{}: bmoney rules [list|add|update|rm]", I18n::text("how_to_use"));
        }
    } else if args[1] == "forecasts" {
        if args[2] == "list" {
            Forecasts::list(storage, args[3..].to_vec(), is_csv);
        } else if args[2] == "add" {
            Forecasts::add(storage, args[3..].to_vec());
        } else if args[2] == "update" {
            Forecasts::update(storage, args[3..].to_vec());
        } else if args[2] == "rm" {
            Forecasts::rm(storage, args[3..].to_vec());
        } else {
            println!("{}: bmoney forecasts [list|add|update|rm]", I18n::text("how_to_use"));
        }
    } else {
     println!("{}: bmoney [accounts|contacts|transactions|tags|rules|forecasts] [action]", I18n::text("how_to_use"));
    }

}
