///
/// Blitz Money
///
/// Module for manange storage of all data
///
/// Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
///

#[derive(Clone, Copy)]
pub struct Storage {
    path: &'static str
}

//
// Storage of bliz money is based in a single file. To
// ensure the integrity of data we need to centralize the
// access to file.
//
static locked_storage: &'static Storage = &Storage { path: "~/.bmoneytmp.bms" };

pub fn get_instance() -> &'static Storage {
    locked_storage.init();

    locked_storage
}

impl Storage {

    fn init(&self) -> bool {
        true
    }

    pub fn start_session(self) -> bool {

        true
    }

}
