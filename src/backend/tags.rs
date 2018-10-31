///
/// Blitz Money
///
/// Backend of module for manange tags
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use backend::storage::*;
use json::JsonValue;

#[derive(Default, Clone, Debug)]
pub struct Tag {
   pub uuid: String,
   pub name: String,
}

impl Model for Tag {

    fn new(row: JsonValue, uuid: String, _storage: &mut Storage, _can_recursive: bool) -> Tag {

        if row["name"].is_null() {
            panic!("Name not found into a row(id {}) tag", uuid);
        }

        Tag {
            uuid: uuid,
            name: row["name"].to_string(),
        }
    }

    fn to_save(self) -> (String, bool, JsonValue) {

        (self.uuid.clone(), self.uuid.is_empty(), object!{
            "name" => self.name,
        })
    }
}

impl Tag {

    // Return a list with all tags
    pub fn get_tags(storage: &mut Storage) -> Vec<Tag> {

        storage.start_section("tags".to_string());

        let mut data = storage.get_section_data("tags".to_string());

        let mut list: Vec<Tag> = vec![];

        while let Ok(line) = data.next::<Tag>() {
            list.push(line);
        }

        return list;
    }

    // Return the tag of id
    pub fn get_tag(storage: &mut Storage, uuid: String) -> Result<Tag, &'static str> {

        storage.start_section("tags".to_string());

        let mut data = storage.get_section_data("tags".to_string());

        if data.find_by_id(uuid) {
            return data.next::<Tag>();
        }

        Err("Tag not found")
    }

    // Save updates, or create new, tag on storage
    pub fn store_tag(storage: &mut Storage, tag: Tag) {

        storage.start_section("tags".to_string());

        let mut data = storage.get_section_data("tags".to_string());

        data.save(tag);
    }

    // Remvoe tag of storage
    pub fn remove_tag(storage: &mut Storage, uuid: String) {

        storage.start_section("tags".to_string());

        let mut data = storage.get_section_data("tags".to_string());

        data.remove_by_id(uuid);
    }
}
