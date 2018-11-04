///
/// Blitz Money
///
/// Frontend/Ui api to buid the interfaces
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use prettytable::*;
use csv::*;
use std::io;
use chrono::NaiveDate;

pub struct Output {
}

pub struct Input {
}

impl Output {

    // Create a new table
    pub fn new_table() -> Table {

        let mut table = Table::new();

        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        table
    }

    // Print the csv on stdout
    pub fn print_table(table: Table, is_csv: bool) {
        if is_csv {
            let wtr = WriterBuilder::new()
                .quote_style(QuoteStyle::NonNumeric)
                .from_writer(io::stdout());

            table.to_csv_writer(wtr)
                .expect("Couldn't generate the csv");
        } else {
            table.printstd();
        }
    }
}


impl Input {

    // Read a value from stdin
    pub fn read(label: String, is_required: bool, current_value: Option<String>) -> String {

        if current_value.is_some() {
            println!("{}: {}(keep blank for skip)", label, current_value.clone().unwrap());
        } else {
            println!("{}:", label);
        }

        let mut value = String::new();
        io::stdin().read_line(&mut value)
            .expect("Failed to read value");

        value = value.trim().to_string();

        // If has current value and user skip
        if current_value.is_some() && value.is_empty() {
            value = current_value.unwrap();
        }

        if is_required && value.is_empty() {
            panic!("This field is required");
        }

        value
    }

    // Read a date value from stdin
    pub fn read_date(mut label: String, is_required: bool, current_value: Option<NaiveDate>) -> Option<NaiveDate> {

        label.push_str("(format YYYY-MM-DD)");

        let mut current:Option<String> = None;
        if current_value.is_some() {
            current = Some(current_value.unwrap().format("%Y-%m-%d").to_string());
        }

        let value = Input::read(label, is_required, current);

        if value.is_empty() && !is_required {
            return None;
        }

        let date = NaiveDate::parse_from_str(&value, "%Y-%m-%d")
            .expect("Couldn't parse the string to date. The format is YYYY-MM-DD");

        Some(date)
    }

    // Read a money value from stdin
    pub fn read_money(mut label: String, is_required: bool, current_value: Option<f32>, currency: String) -> f32 {

        label.push_str("(");
        label.push_str(&currency);
        label.push_str(")");

        let mut current:Option<String> = None;
        if current_value.is_some() {
            current = Some(current_value.unwrap().to_string());
        }

        let value = Input::read(label, is_required, current);

        if value.is_empty() && !is_required {
            return 0.0;
        }

        let money = value.parse::<f32>()
            .expect("Couldn't parse the string to money. The format is 00000.00");

        money
    }

    // Read a option from stdin
    pub fn read_option(mut label: String, is_required: bool, current_value: Option<String>, options: Vec<(String, String)>) -> String {

        if options.len() == 0 {
            panic!("No options avaliable for {}", label);
        }

        label.push_str(", options avaliable:");
        for (i, (_, option)) in options.iter().enumerate() {
            label.push_str(&format!("\n[{}] => {}", i, option));
        }
        label.push_str("\nSelect one or input the full uuid");

        let value = Input::read(label, is_required, current_value);

        // We can't run panic because the user can input a uuid
        let selected = match value.parse::<usize>() {
            Ok(val) => val,
            Err(_e) => options.len() + 1
        };

        // If the user input a valid index
        if selected < options.len() {
            let (uuid, _) = options[selected].clone();
            return uuid;
        }

        value
    }

    // Read options from stdin
    pub fn read_options(mut label: String, is_required: bool, current_value: Vec<String>, options: Vec<(String, String)>) -> Vec<String> {

        if options.len() == 0 {
            panic!("No options avaliable for {}", label);
        }

        label.push_str(", options avaliable:");
        for (i, (_, option)) in options.iter().enumerate() {
            label.push_str(&format!("\n[{}] => {}", i, option));
        }
        label.push_str("\nSelect one or input the full uuid(use ',' to select more)");

        let values = Input::read(label, is_required, None);

        if values.is_empty() {
            return current_value;
        }

        let mut result: Vec<String> = vec!();

        for value in values.split(",") {

            // We can't run panic because the user can input a uuid
            let selected = match value.trim().parse::<usize>() {
                Ok(val) => val,
                Err(_e) => options.len() + 1
            };

            // If the user input a valid index
            if selected < options.len() {
                let (uuid, _) = options[selected].clone();
                result.push(uuid);
            } else {
                result.push(value.trim().to_string());
            }
        }

        result
    }

    // Parse the param to a string
    pub fn param(label: String, is_required: bool, params: Vec<String>, position: usize) -> String {

        if params.len() <= position {
            if is_required {
                panic!("The [{}] is required", label);
            } else {
                return "".to_string();
            }
        }

        let value = params[position].trim().to_string();

        if is_required && value.is_empty() {
            panic!("The [{}] is required", label);
        }

        value
    }

    // Parse the param to a date
    pub fn param_date(label: String, is_required: bool, params: Vec<String>, position: usize) -> Option<NaiveDate> {

        let value = Input::param(label, is_required, params, position);

        if value.is_empty() && !is_required {
            return None;
        }

        let date = NaiveDate::parse_from_str(&value, "%Y-%m-%d")
            .expect("Couldn't parse the string to date. The format is YYYY-MM-DD");

        Some(date)
    }

    // Parse the param to a money value
    pub fn param_money(label: String, is_required: bool, params: Vec<String>, position: usize) -> f32 {

        let value = Input::param(label, is_required, params, position);

        if value.is_empty() && !is_required {
            return 0.0;
        }

        let money = value.parse::<f32>()
            .expect("Couldn't parse the string to money. The format is 00000.00");

        money
    }

    // Return if the param exists on params vec
    // and remove it
    pub fn extract_param(params: &mut Vec<String>, param: String) -> bool {

        for (i, p) in params.clone().iter().enumerate() {
            if p == &param {
                params.remove(i);
                return true;
            }
        }

        false
    }

    // Return value of named param and remove it
    // from vec params. Eg.: --name=value
    pub fn extract_named_param(params: &mut Vec<String>, param: String) -> Option<String> {

        for (i, p) in params.clone().iter().enumerate() {
            if p.starts_with(&param) {
                params.remove(i);
                let vals: Vec<&str> = p.split("=").collect();
                return Some(vals[1].to_string());
            }
        }

        None
    }
}
