pub mod store;

#[derive(Clone)]
pub struct TodoData {
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
