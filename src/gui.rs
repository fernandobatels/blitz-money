#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use qtbindings::*;

pub struct Simple {
    emit: SimpleEmitter,
    message: String,
}

impl SimpleTrait for Simple {
    fn new(emit: SimpleEmitter) -> Simple {
        Simple {
            emit: emit,
            message: "Olá mundo".to_string(),
        }
    }
    fn emit(&self) -> &SimpleEmitter {
        &self.emit
    }
    fn message(&self) -> &str {
        &self.message
    }
    fn set_message(&mut self, value: String) {
        self.message = value;
        self.emit.message_changed();
    }
}

