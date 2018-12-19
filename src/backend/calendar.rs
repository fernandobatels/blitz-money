///
/// Blitz Money
///
/// Backend of module for export and import ical files
///
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use icalendar::*;
use chrono::*;
use backend::transactions::Transaction;
use i18n::*;

pub struct Calendar {
}

impl Calendar {

    // Build the calendar objects for print or storage in a file
    pub fn export(transactions: Vec<Transaction>) -> icalendar::Calendar {

        let mut calendar = icalendar::Calendar::new();

        for tr in transactions {

            let mut event = Event::new();

            if tr.contact.is_some() {
                let contact = tr.contact.clone().unwrap();

                event.summary(&format!("{} - {}", tr.description, contact.name));
                event.location(&contact.city_location);
            } else {
                event.summary(&format!("{} - {}", tr.description, tr.transfer.clone().unwrap().account.unwrap().name));
            }

            event.description(&format!("{lvalue}: {value}\n{laccount}: {account}\n{lobs}: {obs}\n#id: {id}",
                lvalue = I18n::text("transactions_value"),
                value = tr.value_formmated(),
                laccount = I18n::text("transactions_account"),
                account = tr.account.clone().unwrap().name,
                lobs = I18n::text("transactions_observations"),
                obs = tr.observations.clone(),
                id = tr.clone().id()));

            event.uid(&tr.uuid.clone());

            let deadline = tr.deadline.unwrap();

            event.all_day(Utc.ymd(deadline.year(), deadline.month(), deadline.day()));

            calendar.push(event.done());
        }

        calendar
    }
}
