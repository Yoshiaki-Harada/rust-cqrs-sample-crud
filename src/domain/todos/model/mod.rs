pub mod store;

use diesel::{Queryable, Insertable};
use crate::schema::todo;

#[derive(Clone, Queryable)]
pub struct TodoData {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[derive(Insertable)]
#[table_name = "todo"]
pub struct NewTodo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}

pub struct Todo {
    data: TodoData,
}

impl Todo {
    pub(self) fn from_data(data: TodoData) -> Self {
        Todo { data: data }
    }

    // read only, borrow
    pub fn to_data(&self) -> &TodoData {
        &self.data
    }
    // read only, move ownership
    pub fn new_data(self) -> NewTodo {
        NewTodo {
            id: self.data.id,
            title: self.data.title,
            description: self.data.description,
            done: self.data.done,
        }
    }
    pub fn into_data(self) -> TodoData {
        self.data
    }

    pub fn new(id: i32, title: String, description: String, done: bool) -> Self {
        Todo::from_data(TodoData {
            id: id,
            title: title,
            description: description,
            done: done,
        })
    }
}
