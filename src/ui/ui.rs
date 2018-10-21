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

pub struct Output {
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


