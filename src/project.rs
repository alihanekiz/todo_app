use std::fmt;
use serde::Serialize;
use serde::Deserialize;
use crate::todo::Todo;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Project {
    pub name: String,
    pub todos: Vec<Todo>
}

pub enum ProjectOperation {
    Create,
    Delete,
    Open,
    Exit,
    Invalid
}

impl Project {
    pub fn new(name: String) -> Project {
        Project {
            name,
            todos: Vec::new()
        }
    }

    pub fn new_with_todos(name: String, todos: Vec<Todo>) -> Project {
        Project {
            name,
            todos
        }
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Project: {} Todos: {}", self.name, self.todos.len())
    }
}



