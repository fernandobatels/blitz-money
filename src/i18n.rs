///
/// Blitz Money
///
/// For translate de application
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

use json_gettext::*;
use std::sync::Mutex;

pub struct I18n {
}

lazy_static! {
    // yes, this is not a best solution :)
    static ref ctx_i18n: Mutex<Vec<JSONGetText<'static>>> = Mutex::new(vec!());
}

impl I18n {

    // Return the text depending on the language in use
    pub fn text(key: &'static str) -> String {

        let ctx = &ctx_i18n.lock().unwrap()[0];

        match get_text!(ctx, key) {
            Some(text) => text.to_string(),
            None => panic!("The {} not found in current lang", key)
        }
    }

    // Set the default lang for the application
    pub fn config(mut lang: String) {

        let langs_avaliable = ["en_US"];//, "pt_BR"];

        if !langs_avaliable.contains(&lang.as_str()) {
            lang = "en_US".to_string();
        }

        ctx_i18n.lock().unwrap().push(static_json_gettext_build!(
            lang,
            "en_US", "langs/en_US.json"//,
            //"pt_BR", "langs/pt_BR.json"
        ).unwrap());
    }
}
