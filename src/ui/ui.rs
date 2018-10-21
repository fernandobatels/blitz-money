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

    // Parse the param to a string
    pub fn param(label: String, is_required: bool, params: Vec<String>, position: usize) -> String {

        if params.len() < position {
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
}
