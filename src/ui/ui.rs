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
use chrono::{NaiveDate, Local};
use i18n::*;

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
                .expect(&I18n::text("couldnt_generate_the_csv"));
        } else {
            table.printstd();
        }
    }
}


impl Input {

    // Read a value from stdin
    pub fn read(label: String, is_required: bool, current_value: Option<String>) -> String {

        if current_value.is_some() {
            println!("{}: {}({})", label, current_value.clone().unwrap(), I18n::text("keep_blank_for_skip"));
        } else {
            println!("{}:", label);
        }

        let mut value = String::new();
        io::stdin().read_line(&mut value)
            .expect(&I18n::text("failed_to_read_value"));

        value = value.trim().to_string();

        // If has current value and user skip
        if current_value.is_some() && value.is_empty() {
            value = current_value.unwrap();
        }

        if is_required && value.is_empty() {
            panic!(I18n::text("this_field_is_required"));
        }

        value
    }

    // Read a date value from stdin
    pub fn read_date(mut label: String, is_required: bool, current_value: Option<NaiveDate>) -> Option<NaiveDate> {

        label.push_str(&format!("({})", I18n::text("format_date")));

        let mut current:Option<String> = None;
        if current_value.is_some() {
            current = Some(current_value.unwrap().format("%Y-%m-%d").to_string());
        }

        let value = Input::read(label, is_required, current);

        if value.is_empty() && !is_required {
            return None;
        }

        if value == "today".to_string() {
            return Some(Local::now().date().naive_local());
        }

        let date = NaiveDate::parse_from_str(&value, "%Y-%m-%d")
            .expect(&I18n::text("couldnt_parse_the_string_to_date"));

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
            .expect(&I18n::text("couldnt_parse_the_string_to_money"));

        money
    }

    // Read a integer value from stdin
    pub fn read_int(label: String, is_required: bool, current_value: Option<i32>) -> i32 {

        let mut current:Option<String> = None;
        if current_value.is_some() {
            current = Some(current_value.unwrap().to_string());
        }

        let value = Input::read(label, is_required, current);

        if value.is_empty() && !is_required {
            return 0;
        }

        let int = value.parse::<i32>()
            .expect(&I18n::text("couldnt_parse_the_string_to_integer"));

        int
    }

    // Build the str for display options
    fn str_for_display_options(options: Vec<(String, String)>, filter: Option<String>) -> String {

        let mut label = String::new();
        let mut cols = 0;

        for (i, (_, option)) in options.iter().enumerate() {

            if filter.clone().is_none() || option.to_lowercase().contains(filter.clone().unwrap().as_str()) {

                if cols == 3 {
                    label.push_str("\n");
                    cols = 0;
                }
                label.push_str(&format!("[{:2}] => {:50}", i, option));
                cols += 1;
            }
        }
        label.push_str(&I18n::text("select_one_input_the_full_uuid_or_search"));

        label
    }

    // Display the options and provide a interface to search him
    fn select_options(mut label: String, is_required: bool, current_value: Option<String>, options: Vec<(String, String)>) -> String {

        label.push_str(&I18n::text("options_avaliable"));
        label.push_str(&Input::str_for_display_options(options.clone(), None));

        let mut value: String;

        loop {

            value = Input::read(label.clone(), is_required, current_value.clone());

            // Searching a option
            if value.starts_with("\\") {

                label = I18n::text("options_found");

                value = value.replace("\\", "").trim().to_lowercase().to_string();

                label.push_str(&Input::str_for_display_options(options.clone(), Some(value)));

            } else {
                break;
            }

        }

        value
    }

    // Read a option from stdin
    pub fn read_option(label: String, is_required: bool, current_value: Option<String>, mut options: Vec<(String, String)>) -> String {

        if options.len() == 0 {
            if !is_required {
                return "".to_string();
            }

            panic!("{} {}", I18n::text("no_options_avaliable_for"), label);
        }

        options.sort_by( | (_, a), (_, b) | a.cmp(&b) );

        let value = Input::select_options(label, is_required, current_value, options.clone());

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
    pub fn read_options(label: String, is_required: bool, current_value: Vec<String>, mut options: Vec<(String, String)>) -> Vec<String> {

        if options.len() == 0 {
            if !is_required {
                return vec!();
            }

            panic!("{} {}", I18n::text("no_options_avaliable_for"), label);
        }

        options.sort_by( | (_, a), (_, b) | a.cmp(&b) );

        let values = Input::select_options(label, is_required, None, options.clone());

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
                panic!("{} {}", I18n::text("the__is_required"), label);
            } else {
                return "".to_string();
            }
        }

        let value = params[position].trim().to_string();

        if is_required && value.is_empty() {
            panic!("{} {}", I18n::text("the__is_required"), label);
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
            .expect(&I18n::text("couldnt_parse_the_string_to_date"));

        Some(date)
    }

    // Parse the param to a money value
    pub fn param_money(label: String, is_required: bool, params: Vec<String>, position: usize) -> f32 {

        let value = Input::param(label, is_required, params, position);

        if value.is_empty() && !is_required {
            return 0.0;
        }

        let money = value.parse::<f32>()
            .expect(&I18n::text("couldnt_parse_the_string_to_money"));

        money
    }

    // Parse the param to a integer value
    pub fn param_int(label: String, is_required: bool, params: Vec<String>, position: usize) -> i32 {

        let value = Input::param(label, is_required, params, position);

        if value.is_empty() && !is_required {
            return 0;
        }

        let int = value.parse::<i32>()
            .expect(&I18n::text("couldnt_parse_the_string_to_integer"));

        int
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
