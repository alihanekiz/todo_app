use std::fmt;
use serde::Serialize;
use serde::Deserialize;
use colored::Colorize;


#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Todo {
    pub done: bool,
    pub desc: String
}

pub enum TodoOperation {
    Delete,
    Mark
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status: String = match self.done {
            true => String::from("[x]"),
            false => String::from("[]")
        };
        match self.done {
            true => write!(f, "{} {}", status.green(), self.desc.green()),
            false => write!(f, "{} {}", status.red(), self.desc.red())
        }
    }
}



