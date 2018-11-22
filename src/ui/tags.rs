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

pub struct Tags {}

impl Tags {

    // List of user tags
    pub fn list(mut storage: Storage, _params: Vec<String>, is_csv: bool) {

        let tags = Tag::get_tags(&mut storage);
        let mut table = Output::new_table();

        table.set_titles(row![b->"Name", b->"#id"]);

        for tag in tags {

            table.add_row(row![
                tag.name,
                tag.uuid
            ]);
        }

        Output::print_table(table, is_csv);
    }

    // Create new tag
    pub fn add(mut storage: Storage, params: Vec<String>) {

        if params.len() == 1 && params[0] != "-i"{
            // Shell mode

            let name = Input::param("Tag name".to_string(), true, params.clone(), 0);

            Tag::store_tag(&mut storage, Tag {
                uuid: "".to_string(),
                name: name,
            });
        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let name = Input::read("Tag name".to_string(), true, None);

            Tag::store_tag(&mut storage, Tag {
                uuid: "".to_string(),
                name: name,
            });
        } else {
            // Help mode
            println!("How to use: bmoney tags add [name]");
            println!("Or with interactive mode: bmoney tags add -i")
        }
    }

    // Update a existing tag
    pub fn update(mut storage: Storage, params: Vec<String>) {

        if params.len() == 3 {
            // Shell mode

            let mut tag = Tag::get_tag(&mut storage, params[0].trim().to_string())
                .expect("Tag not found");

            if params[1] == "name" {
                tag.name = Input::param("Tag name".to_string(), true, params.clone(), 2);
            } else {
                panic!("Field not found!");
            }

            Tag::store_tag(&mut storage, tag);

        } else if params.len() > 0 && params[0] == "-i" {
            // Interactive mode

            let id = Input::read("Tag id".to_string(), true, None);

            let mut tag = Tag::get_tag(&mut storage, id)
                .expect("Tag not found");

            tag.name = Input::read("Tag name".to_string(), true, Some(tag.name));

            Tag::store_tag(&mut storage, tag);
        } else {
            // Help mode
            println!("How to use: bmoney tags update [id] [name] [value]");
            println!("Or with interactive mode: bmoney tags update -i")
        }
    }

    // Remove a existing tag
    pub fn rm(mut storage: Storage, params: Vec<String>) {

        if params.len() == 1 {
            // Shell mode

            Tag::remove_tag(&mut storage, params[0].trim().to_string());
        } else {
            // Help mode
            println!("How to use: bmoney tags rm [id]");
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use uuid::Uuid;
    use std::process::Command;
    use assert_cmd::prelude::*;

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
    fn tags_list() {

        let (path_str, uuids) = populate();

        let mut main = Command::main_binary().unwrap();

        main.arg("tags")
            .arg("list")
            .arg("--storage-file=".to_owned() + &path_str)
            .arg("--use-csv");

        let mut stdout = String::from("\"Name\",\"#id\"\n");

        stdout.push_str(&format!("\"tag 4\",\"{}\"\n", uuids[3]));
        stdout.push_str(&format!("\"tag 3\",\"{}\"\n", uuids[2]));
        stdout.push_str(&format!("\"tag 2\",\"{}\"\n", uuids[1]));
        stdout.push_str(&format!("\"tag 1\",\"{}\"\n", uuids[0]));

        main.assert()
            .success()
            .stdout(stdout.to_owned());
    }

}
