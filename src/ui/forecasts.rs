///
/// Blitz Money
///
/// Frontend/Ui of module for manange forecast values of months
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use backend::forecasts::Forecast;
use backend::tags::Tag;
use backend::accounts::Account;
use backend::storage::Storage;
use ui::ui::*;
use i18n::*;

pub struct Forecasts {}

impl Forecasts {

    // List of user forecasts
    pub fn list(mut storage: Storage, _params: Vec<String>, is_csv: bool) {

        let forecasts = Forecast::get_forecasts(&mut storage);
        let mut table = Output::new_table();

        table.set_titles(row![b->I18n::text("forecasts_tag"), b->I18n::text("forecasts_account"), b->I18n::text("forecasts_value"), b->"#id"]);

        for forecast in forecasts {

            let account = forecast.clone().account.unwrap().clone();

            let mut row = table.add_row(row![
                forecast.clone().tag.unwrap().name,
                account.name,
                Fg->account.format_value(forecast.value),
                forecast.clone().id()
            ]);

            if forecast.value < 0.0 {
                row.set_cell(cell!(Fr->account.format_value(forecast.value)), 2)
                    .expect(&I18n::text("forecasts_unable_to_set_value"));
            }
        }

        Output::print_table(table, is_csv);
    }

    // Create new forecast
    pub fn add(mut storage: Storage, params: Vec<String>) {

        if params.len() >= 3 || (params.len() == 1 && params[0] == "-i") {

            let tag_uuid;
            let account_uuid;
            let value;

            if params.len() == 3 {
                // Shell mode

                tag_uuid = Input::param(I18n::text("forecasts_tag"), true, params.clone(), 0);
                account_uuid = Input::param(I18n::text("forecasts_account"), true, params.clone(), 1);
                value = Input::param_money(I18n::text("forecasts_value"), true, params.clone(), 2);

            } else {
                // Interactive mode

                let mut tags: Vec<(String, String)> = vec![];
                for tag in Tag::get_tags(&mut storage) {
                    tags.push((tag.uuid, tag.name));
                }
                tag_uuid = Input::read_option(I18n::text("forecasts_tag"), true, None, tags);

                let mut accounts: Vec<(String, String)> = vec![];
                for ac in Account::get_accounts(&mut storage) {
                    accounts.push((ac.uuid, ac.name));
                }
                account_uuid = Input::read_option(I18n::text("forecasts_account"), true, None, accounts);

                value = Input::read_money(I18n::text("forecasts_value"), true, None, "".to_string());
            }

            let tag = Some(Tag::get_tag(&mut storage, tag_uuid).unwrap());
            let account = Some(Account::get_account(&mut storage, account_uuid).unwrap());

            Forecast::store_forecast(&mut storage, Forecast {
                uuid: "".to_string(),
                account: account,
                value: value.clone(),
                tag: tag
            });
        } else {
            // Help mode
            println!("{}", I18n::text("forecasts_how_to_use_add"));
        }
    }

    // Update a existing forecast
    pub fn update(mut storage: Storage, params: Vec<String>) {

        if params.len() == 3 {
            // Shell mode

            let mut forecast = Forecast::get_forecast(&mut storage, params[0].trim().to_string())
                .expect(&I18n::text("forecasts_not_found"));

            if params[1] == "tag" {
                let tag_uuid = Input::param(I18n::text("forecasts_tag"), true, params.clone(), 2);
                forecast.tag = Some(Tag::get_tag(&mut storage, tag_uuid).unwrap());
            } else if params[1] == "account" {
                let account_uuid = Input::param(I18n::text("forecasts_account"), true, params.clone(), 2);
                forecast.account = Some(Account::get_account(&mut storage, account_uuid).unwrap());
            } else if params[1] == "value" {
                forecast.value = Input::param_money(I18n::text("forecasts_value"), true, params.clone(), 2);
            } else {
                panic!(I18n::text("field_not_found"));
            }

            Forecast::store_forecast(&mut storage, forecast);

        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let id = Input::read("#id".to_string(), true, None);

            let mut forecast = Forecast::get_forecast(&mut storage, id)
                .expect(&I18n::text("forecasts_not_found"));

            let mut tags: Vec<(String, String)> = vec![];
            for tag in Tag::get_tags(&mut storage) {
                tags.push((tag.uuid, tag.name));
            }
            let tag_uuid = Input::read_option(I18n::text("forecasts_tag"), true, Some(forecast.tag.unwrap().uuid), tags);
            forecast.tag = Some(Tag::get_tag(&mut storage, tag_uuid).unwrap());

            let mut accounts: Vec<(String, String)> = vec![];
            for ac in Account::get_accounts(&mut storage) {
                accounts.push((ac.uuid, ac.name));
            }
            let account_uuid = Input::read_option(I18n::text("forecasts_account"), true, Some(forecast.account.unwrap().uuid), accounts);
            forecast.account = Some(Account::get_account(&mut storage, account_uuid).unwrap());

            forecast.value = Input::read_money(I18n::text("forecasts_value"), true, Some(forecast.value), "".to_string());

            Forecast::store_forecast(&mut storage, forecast);

        } else {
            // Help mode
            println!("{}", I18n::text("forecasts_how_to_use_update"));
        }
    }

    // Remove a existing forecast
    pub fn rm(mut storage: Storage, params: Vec<String>) {

        if params.len() == 1 {
            // Shell mode

            Forecast::remove_forecast(&mut storage, params[0].trim().to_string());

        } else {
            // Help mode
            println!("{}", I18n::text("forecasts_how_to_use_rm"));
        }
    }
}
