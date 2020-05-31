use crate::domain::todos::{Todo, TodoStore};
use auto_impl::auto_impl;
use crate::domain::todos::model::store::PostgresDb;
use crate::domain::Resolver;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetTodoByTitle {
    pub title: String
}

#[auto_impl(Fn)]
pub trait GetTodoByTitleQuery {
    fn get_todo_by_title(&self, query: GetTodoByTitle) -> Vec<Todo>;
}

pub(in crate::domain) fn get_todo_by_title_query(store: impl TodoStore, pool: PostgresDb) -> impl GetTodoByTitleQuery {
    move |query: GetTodoByTitle| {
        store.get_todo_by_title(&query.title, &pool.get().expect("We can't get connection"))
    }
}

impl Resolver {
    pub fn get_todo_by_title_query(&self) -> impl GetTodoByTitleQuery {
        let store = self.todos().todo_store();
        let conn = self.todos().get_pool();

        get_todo_by_title_query(store, conn)
    }
}