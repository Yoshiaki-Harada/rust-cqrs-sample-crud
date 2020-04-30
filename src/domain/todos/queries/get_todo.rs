use crate::domain::resolver::Resolver;
use crate::domain::todos::TodoStore;
use crate::domain::todos::Todo;
use serde::Deserialize;
use auto_impl::auto_impl;
#[derive(Deserialize)]
pub struct GetTodo {
    pub id: i32
}

#[auto_impl(Fn)]
pub trait GetTodoQuery {
    fn get_todo(&self, query: GetTodo)->Todo;
}

pub (in crate::domain) fn get_todo_query(store: impl TodoStore) -> impl GetTodoQuery {
    move |query: GetTodo| {
        store.get_todo(query.id)
    }
}

impl Resolver {
    pub fn get_todo_query(&self)-> impl GetTodoQuery {
        let store = self.todos().todo_store();
        get_todo_query(store)
    }
}