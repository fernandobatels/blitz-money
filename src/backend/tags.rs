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

    // Short version of the uuid
    pub fn id(self) -> String {
        Data::uuid_to_id(self.uuid)
    }

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
    pub fn store_tag(storage: &mut Storage, tag: Tag) -> String {

        storage.start_section("tags".to_string());

        let mut data = storage.get_section_data("tags".to_string());

        data.save(tag)
    }

    // Remvoe tag of storage
    pub fn remove_tag(storage: &mut Storage, uuid: String) {

        storage.start_section("tags".to_string());

        let mut data = storage.get_section_data("tags".to_string());

        data.remove_by_id(uuid);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use uuid::Uuid;

    fn populate() -> String {

        let path = "/tmp/bmoney-".to_owned() + &Uuid::new_v4().to_string();

        let mut st = Storage { path_str: path.clone(), file: None, lines: Vec::new() };

        assert!(st.start_section("tags".to_string()));

        let mut data = st.get_section_data("tags".to_string());

        data.save(Tag { uuid: "".to_string(), name: "tag 1".to_string() });
        data.save(Tag { uuid: "".to_string(), name: "tag 2".to_string() });
        data.save(Tag { uuid: "".to_string(), name: "tag 3".to_string() });
        data.save(Tag { uuid: "".to_string(), name: "tag 4".to_string() });

        path
    }

    #[test]
    fn get_tags() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new() };

        let tags = Tag::get_tags(&mut st);

        assert_eq!(tags[0].name, "tag 4".to_string());
        assert_eq!(tags[1].name, "tag 3".to_string());
        assert_eq!(tags[2].name, "tag 2".to_string());
        assert_eq!(tags[3].name, "tag 1".to_string());
    }

    #[test]
    fn get_tag() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new() };

        let tags = Tag::get_tags(&mut st);

        let uuid = tags[0].uuid.clone();

        let tag = Tag::get_tag(&mut st, uuid);

        assert!(tag.is_ok());
        assert_eq!(tag.unwrap().name, "tag 4".to_string());

        let tage = Tag::get_tag(&mut st, "NOOOO".to_string());

        assert!(tage.is_err());
    }


    #[test]
    fn store_tag() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new() };

        Tag::store_tag(&mut st, Tag { uuid: "".to_string(), name: "tag 5".to_string() });

        let tags = Tag::get_tags(&mut st);

        assert_eq!(tags[0].name, "tag 5".to_string());
        assert_eq!(tags[1].name, "tag 4".to_string());
    }

    #[test]
    fn remove_tag() {

        let mut st = Storage { path_str: populate(), file: None, lines: Vec::new() };

        let tags = Tag::get_tags(&mut st);

        let uuid = tags[0].uuid.clone();

        let tag = Tag::get_tag(&mut st, uuid.clone());

        assert!(tag.is_ok());

        Tag::remove_tag(&mut st, uuid.clone());

        let tage = Tag::get_tag(&mut st, uuid.clone());

        assert!(tage.is_err());
    }
}
