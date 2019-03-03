///
/// Blitz Money
///
/// Frontend/Ui of module for manange rules of padronizations fields
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use backend::rules::Rule;
use backend::contacts::Contact;
use backend::tags::Tag;
use backend::storage::Storage;
use ui::ui::*;
use i18n::*;

pub struct Rules {}

impl Rules {

    // List of user rules
    pub fn list(mut storage: Storage, _params: Vec<String>, is_csv: bool) {

        let rules = Rule::get_rules(&mut storage);
        let mut table = Output::new_table();

        table.set_titles(row![b->I18n::text("rules_term"), b->I18n::text("rules_expected_value"), b->I18n::text("rules_description"), b->I18n::text("rules_contact"), b->I18n::text("rules_tags"), b->"#id"]);

        for rule in rules {

            let tags: Vec<String> = rule.tags
                .iter()
                .map(|tag| tag.name.clone())
                .collect();

            let mut row = table.add_row(row![
                rule.term,
                "",
                rule.description,
                "",
                tags.join(", "),
                rule.clone().id()
            ]);

            if rule.expected_value.is_some() {
                row.set_cell(cell!(rule.expected_value.unwrap()), 1)
                    .expect(&I18n::text("rules_unable_to_set_expected_value"));
            }

            if rule.contact.is_some() {
                row.set_cell(cell!(rule.contact.clone().unwrap().name), 3)
                    .expect(&I18n::text("rules_unable_to_set_contact"));
            }
        }

        Output::print_table(table, is_csv);
    }

    // Create new rule
    pub fn add(mut storage: Storage, params: Vec<String>) {

        if params.len() == 5 {
            // Shell mode

            let term = Input::param(I18n::text("rules_term"), true, params.clone(), 0);

            let mut expected_value: Option<f32> = None;
            let get_expected = Input::param_money(I18n::text("rules_expected_value"), false, params.clone(), 1);
            if get_expected != 0.0 {
                expected_value = Some(get_expected);
            }

            let description = Input::param(I18n::text("rules_description"), true, params.clone(), 2);

            let contact_uuid = Input::param(I18n::text("rules_contact"), false, params.clone(), 3);

            let contact = match Contact::get_contact(&mut storage, contact_uuid.clone()) {
                Ok(con) => Some(con),
                Err(_)  => None
            };

            let mut tags: Vec<Tag> = vec!();

            let tags_str = Input::param(I18n::text("rules_tags"), false, params.clone(), 4);

            if !tags_str.is_empty() {
                for tag in tags_str.split(",") {
                    tags.push(
                        Tag::get_tag(&mut storage, tag.to_string())
                            .expect(&I18n::text("tags_not_found"))
                    );
                }
            }

            Rule::store_rule(&mut storage, Rule {
                uuid: "".to_string(),
                description: description,
                expected_value: expected_value.clone(),
                term: term,
                contact: contact.clone(),
                tags: tags
            });
        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let term = Input::read(I18n::text("rules_term"), true, None);

            let mut expected_value: Option<f32> = None;
            let get_expected = Input::read_money(I18n::text("rules_expected_value"), false, None, "".to_string());
            if get_expected != 0.0 {
                expected_value = Some(get_expected);
            }

            let description = Input::read(I18n::text("rules_description"), true, None);

            let mut contacts: Vec<(String, String)> = vec![];
            for co in Contact::get_contacts(&mut storage) {
                contacts.push((co.uuid, co.name));
            }

            let contact_uuid = Input::read_option(I18n::text("rules_contact"), false, None, contacts);

            let contact = match Contact::get_contact(&mut storage, contact_uuid.clone()) {
                Ok(con) => Some(con),
                Err(_)  => None
            };

            let mut tags_ops: Vec<(String, String)> = vec![];
            for tag in Tag::get_tags(&mut storage) {
                tags_ops.push((tag.uuid, tag.name));
            }

            let tags: Vec<Tag> = Input::read_options(I18n::text("rules_tags"), false, vec![], tags_ops)
                .iter()
                .map(
                    |tag| Tag::get_tag(&mut storage, tag.to_string())
                                .expect(&I18n::text("tags_not_found"))
                )
                .collect();

            Rule::store_rule(&mut storage, Rule {
                uuid: "".to_string(),
                description: description,
                expected_value: expected_value.clone(),
                term: term,
                contact: contact.clone(),
                tags: tags
            });
        } else {
            // Help mode
            println!("{}", I18n::text("rules_how_to_use_add"));
        }
    }


    // Update a existing rule
    pub fn update(mut storage: Storage, params: Vec<String>) {

        if params.len() == 3 {
            // Shell mode

            let mut rule = Rule::get_rule(&mut storage, params[0].trim().to_string())
                .expect(&I18n::text("rules_not_found"));

            if params[1] == "term" {
                rule.term = Input::param(I18n::text("rules_term"), true, params.clone(), 2);
            } else if params[1] == "description" {
                rule.description = Input::param(I18n::text("rules_description"), true, params.clone(), 2);
            } else if params[1] == "expected" {
                let get_expected = Input::param_money(I18n::text("rules_expected_value"), false, params.clone(), 2);
                if get_expected != 0.0 {
                    rule.expected_value = Some(get_expected);
                } else {
                    rule.expected_value = None;
                }
            } else if params[1] == "contact" {
                let contact_uuid = Input::param(I18n::text("rules_contact"), true, params.clone(), 2);
                rule.contact = Some(Contact::get_contact(&mut storage, contact_uuid).unwrap());
            } else if params[1] == "tags" {
                let tags_str = Input::param(I18n::text("rules_tags"), false, params.clone(), 2);
                rule.tags = vec![];

                if !tags_str.is_empty() {
                    for tag in tags_str.split(",") {
                        rule.tags.push(
                            Tag::get_tag(&mut storage, tag.to_string())
                                .expect(&I18n::text("tags_not_found"))
                        );
                    }
                }

            } else {
                panic!(I18n::text("field_not_found"));
            }

            Rule::store_rule(&mut storage, rule);

        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let id = Input::read("#id".to_string(), true, None);

            let mut rule = Rule::get_rule(&mut storage, id)
                .expect(&I18n::text("rules_not_found"));

            rule.term = Input::read(I18n::text("rules_term"), true, Some(rule.term));

            let get_expected = Input::read_money(I18n::text("rules_expected_value"), false, rule.expected_value, "".to_string());
            if get_expected != 0.0 {
                rule.expected_value = Some(get_expected);
            } else {
                rule.expected_value = None;
            }

            rule.description = Input::read(I18n::text("rules_description"), true, Some(rule.description));

            let mut contacts: Vec<(String, String)> = vec![];
            for co in Contact::get_contacts(&mut storage) {
                contacts.push((co.uuid, co.name));
            }

            let contact_uuid = Input::read_option(I18n::text("rules_contact"), true, Some(rule.contact.clone().unwrap().uuid), contacts);
            rule.contact = match Contact::get_contact(&mut storage, contact_uuid.clone()) {
                Ok(con) => Some(con),
                Err(_)  => None
            };

            let mut tags_ops: Vec<(String, String)> = vec![];
            for tag in Tag::get_tags(&mut storage) {
                tags_ops.push((tag.uuid, tag.name));
            }

            let current_tags: Vec<String> = rule.tags.clone()
                .iter()
                .map(|tag| tag.uuid.clone())
                .collect();

            rule.tags = Input::read_options(I18n::text("rules_tags"), false, current_tags, tags_ops)
                .iter()
                .map(
                    |tag| Tag::get_tag(&mut storage, tag.to_string())
                                .expect(&I18n::text("tags_not_found"))
                )
                .collect();

            Rule::store_rule(&mut storage, rule);

        } else {
            // Help mode
            println!("{}", I18n::text("rules_how_to_use_update"));
        }
    }

    // Remove a existing rule
    pub fn rm(mut storage: Storage, params: Vec<String>) {

        if params.len() == 1 {
            // Shell mode

            Rule::remove_rule(&mut storage, params[0].trim().to_string());

        } else {
            // Help mode
            println!("{}", I18n::text("rules_how_to_use_rm"));
        }
    }
}
