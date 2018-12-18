///
/// Blitz Money
///
/// Frontend/Ui of module for manange tags of user
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use backend::tags::Tag;
use backend::storage::Storage;
use ui::ui::*;
use i18n::*;

pub struct Tags {}

impl Tags {

    // List of user tags
    pub fn list(mut storage: Storage, _params: Vec<String>, is_csv: bool) {

        let tags = Tag::get_tags(&mut storage);
        let mut table = Output::new_table();

        table.set_titles(row![b->I18n::text("tags_name"), b->"#id"]);

        for tag in tags {

            table.add_row(row![
                tag.name,
                tag.clone().id()
            ]);
        }

        Output::print_table(table, is_csv);
    }

    // Create new tag
    pub fn add(mut storage: Storage, params: Vec<String>) {

        if params.len() == 1 && params[0] != "-i"{
            // Shell mode

            let name = Input::param(I18n::text("tags_name").to_string(), true, params.clone(), 0);

            Tag::store_tag(&mut storage, Tag {
                uuid: "".to_string(),
                name: name,
            });
        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let name = Input::read(I18n::text("tags_name"), true, None);

            Tag::store_tag(&mut storage, Tag {
                uuid: "".to_string(),
                name: name,
            });
        } else {
            // Help mode
            println!("{}", I18n::text("tags_how_to_use_add"));
        }
    }

    // Update a existing tag
    pub fn update(mut storage: Storage, params: Vec<String>) {

        if params.len() == 3 {
            // Shell mode

            let mut tag = Tag::get_tag(&mut storage, params[0].trim().to_string())
                .expect(&I18n::text("tags_not_found"));

            if params[1] == "name" {
                tag.name = Input::param(I18n::text("tags_name"), true, params.clone(), 2);
            } else {
                panic!(I18n::text("field_not_found"));
            }

            Tag::store_tag(&mut storage, tag);

        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let id = Input::read("#id".to_string(), true, None);

            let mut tag = Tag::get_tag(&mut storage, id)
                .expect(&I18n::text("tags_not_found"));

            tag.name = Input::read(I18n::text("tags_name"), true, Some(tag.name));

            Tag::store_tag(&mut storage, tag);
        } else {
            // Help mode
            println!("{}", I18n::text("tags_how_to_use_update"));
        }
    }

    // Remove a existing tag
    pub fn rm(mut storage: Storage, params: Vec<String>) {

        if params.len() == 1 {
            // Shell mode

            Tag::remove_tag(&mut storage, params[0].trim().to_string());
        } else {
            // Help mode
            println!("{}", I18n::text("tags_how_to_use_rm"));
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use uuid::Uuid;
    use std::process::Command;
    use assert_cmd::prelude::*;
    use backend::storage::Data;

    fn populate() -> (String, Vec<String>) {

        let path = "/tmp/bmoney-ui-".to_owned() + &Uuid::new_v4().to_string();

        let mut st = Storage { path_str: path.clone(), file: None, lines: Vec::new() };

        let mut uuids: Vec<String> = vec![];

        uuids.push(Tag::store_tag(&mut st, Tag { uuid: "".to_string(), name: "tag 1".to_string() }));
        uuids.push(Tag::store_tag(&mut st, Tag { uuid: "".to_string(), name: "tag 2".to_string() }));
        uuids.push(Tag::store_tag(&mut st, Tag { uuid: "".to_string(), name: "tag 3".to_string() }));
        uuids.push(Tag::store_tag(&mut st, Tag { uuid: "".to_string(), name: "tag 4".to_string() }));

        (path, uuids)
    }

    #[test]
    fn tags_insert() {

        let (path_str, _uuids) = populate();

        let mut main = Command::main_binary().unwrap();

        main.arg("tags")
            .arg("add")
            .arg("--storage-file=".to_owned() + &path_str)
            .arg("\"My new tag\"");

        main.assert()
            .success();
    }

    #[test]
    fn tags_list() {

        let (path_str, uuids) = populate();

        let mut main = Command::main_binary().unwrap();

        main.arg("tags")
            .arg("list")
            .arg("--storage-file=".to_owned() + &path_str)
            .arg("--use-csv");

        let mut stdout = String::from("\"Tag name\",\"#id\"\n");

        stdout.push_str(&format!("\"tag 4\",\"{}\"\n", Data::uuid_to_id(uuids[3].clone())));
        stdout.push_str(&format!("\"tag 3\",\"{}\"\n", Data::uuid_to_id(uuids[2].clone())));
        stdout.push_str(&format!("\"tag 2\",\"{}\"\n", Data::uuid_to_id(uuids[1].clone())));
        stdout.push_str(&format!("\"tag 1\",\"{}\"\n", Data::uuid_to_id(uuids[0].clone())));

        main.assert()
            .success()
            .stdout(stdout.to_owned());
    }

}
