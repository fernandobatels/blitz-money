///
/// Blitz Money
///
/// Backend of module for manange rules padronizations of imported fields
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use backend::storage::*;
use backend::contacts::*;
use backend::tags::*;
use backend::transactions::*;
use json::JsonValue;

#[derive(Default, Clone, Debug)]
pub struct Rule {
   pub uuid: String,
   pub term: String,
   pub expected_value: Option<f32>,
   pub description: String,
   pub contact: Option<Contact>,
   pub tags: Vec<Tag>
}

impl Model for Rule {

    fn new(row: JsonValue, uuid: String, storage: &mut Storage, _can_recursive: bool) -> Rule {

        if row["term"].is_null() {
            panic!("Term not found into a row(id {}) rule", uuid);
        }

        if row["description"].is_null() {
            panic!("Description not found into a row(id {}) rule", uuid);
        }

        let mut rule = Rule {
            uuid: uuid,
            term: row["term"].to_string(),
            expected_value: None,
            description: row["description"].to_string(),
            contact: None,
            tags: vec!()
        };

        if !row["expected_value"].is_empty() {
            rule.expected_value = Some(row["expected_value"].as_f32().unwrap());
        }

        if !row["contact"].is_empty() {
            rule.contact = Some(Contact::get_contact(storage, row["contact"].to_string()).unwrap());
        }

        if !row["tags"].is_empty() {
            for stag in row["tags"].members() {
                if let Ok(tag) = Tag::get_tag(storage, stag.to_string()) {
                    rule.tags.push(tag);
                }
            }
        }

        rule
    }

    fn to_save(self) -> (String, bool, JsonValue) {

        let mut ob = object!{
            "term" => self.term,
            "description" => self.description
        };

        if self.contact.is_some() {
            ob["contact"] = self.contact.unwrap().uuid.into();
        }

        if  self.expected_value.is_some() {
            ob["expected_value"] = self.expected_value.unwrap().into();
        }

        if self.tags.len() > 0 {
            let tags: Vec<String> = self.tags
                .iter()
                .map(|tag| tag.uuid.clone())
                .collect();

            ob["tags"] = tags.into();
        }

        (self.uuid.clone(), self.uuid.is_empty(), ob)
    }
}

impl Rule {

    // Short version of the uuid
    pub fn id(self) -> String {
        Data::uuid_to_id(self.uuid)
    }

    // Return a list with all rules
    pub fn get_rules(storage: &mut Storage) -> Vec<Rule> {

        storage.start_section("rules".to_string());

        let mut data = storage.get_section_data("rules".to_string());

        let mut list: Vec<Rule> = vec![];

        while let Ok(line) = data.next::<Rule>() {
            list.push(line);
        }

        return list;
    }

    // Return the rule of id
    pub fn get_rule(storage: &mut Storage, uuid: String) -> Result<Rule, &'static str> {

        storage.start_section("rules".to_string());

        let mut data = storage.get_section_data("rules".to_string());

        if data.find_by_id(uuid) {
            return data.next::<Rule>();
        }

        Err("Rule not found")
    }

    // Save updates, or create new, rule on storage
    pub fn store_rule(storage: &mut Storage, rule: Rule) {

        storage.start_section("rules".to_string());

        let mut data = storage.get_section_data("rules".to_string());

        data.save(rule);
    }

    // Remvoe rule of storage
    pub fn remove_rule(storage: &mut Storage, uuid: String) {

        storage.start_section("rules".to_string());

        let mut data = storage.get_section_data("rules".to_string());

        data.remove_by_id(uuid);
    }

    // Check the description and apply the first rule with term match
    pub fn apply_rules(storage: &mut Storage, transaction: &mut Transaction) -> bool {

        let rules = Rule::get_rules(storage);

        for rule in rules {
            if transaction.description.to_lowercase().contains(rule.term.to_lowercase().as_str()) {

                if rule.expected_value.is_some() {
                    if rule.expected_value.unwrap() != transaction.value {
                        continue;
                    }
                }

                transaction.description = rule.description.clone();

                transaction.contact = rule.contact.clone();

                transaction.tags = rule.tags.clone();

                return true;
            }
        }

        false
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use uuid::Uuid;
    use std::collections::HashMap;
    use std::cell::RefCell;

    fn populate() -> String {

        let path = "/tmp/bmoney-".to_owned() + &Uuid::new_v4().to_string();

        let mut st = Storage { path_str: path.clone(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new()) };

        Contact::store_contact(&mut st, Contact { uuid: "".to_string(), name: "contact 1".to_string(), city_location: "city A".to_string() });
        Contact::store_contact(&mut st, Contact { uuid: "".to_string(), name: "contact 2".to_string(), city_location: "city B".to_string() });

        let contacts = Contact::get_contacts(&mut st);

        assert!(st.start_section("rules".to_string()));

        let mut data = st.get_section_data("rules".to_string());

        data.save(Rule {
            uuid: "".to_string(),
            description: "rule 1".to_string(),
            term: "term A".to_string(),
            expected_value: None,
            contact: Some(contacts[0].clone()),
            tags: vec!()
        });
        data.save(Rule {
            uuid: "".to_string(),
            description: "rule 2".to_string(),
            term: "term B".to_string(),
            expected_value: None,
            contact: Some(contacts[1].clone()),
            tags: vec!()
        });
        data.save(Rule {
            uuid: "".to_string(),
            description: "rule 3".to_string(),
            term: "term D".to_string(),
            expected_value: None,
            contact: Some(contacts[1].clone()),
            tags: vec!()
        });
        data.save(Rule {
            uuid: "".to_string(),
            description: "rule 4".to_string(),
            term: "term B".to_string(),
            expected_value: None,
            contact: None,
            tags: vec!()
        });

        path
    }

    #[test]
    fn get_rules() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new()) };

        let rules = Rule::get_rules(&mut st);

        assert_eq!(rules[0].description, "rule 4".to_string());
        assert_eq!(rules[0].term, "term B".to_string());
        assert!(!rules[0].contact.is_some());
        assert_eq!(rules[0].tags.len(), 0);

        assert_eq!(rules[3].description, "rule 1".to_string());
        assert_eq!(rules[3].term, "term A".to_string());
        assert!(rules[3].contact.is_some());
        assert_eq!(rules[3].tags.len(), 0);
    }

    #[test]
    fn apply_rules() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new(), index: RefCell::new(HashMap::new()) };

        let mut tr = Transaction {
            description: "more term b and other".to_string(),
            value: 150.0,
            ..Default::default()
        };

        assert!(Rule::apply_rules(&mut st, &mut tr));

        assert_eq!(tr.description, "rule 4".to_string());

        let mut tr2 = Transaction {
            description: "more term not found and other".to_string(),
            value: 10.0,
            ..Default::default()
        };

        assert!(!Rule::apply_rules(&mut st, &mut tr2));
    }
}
