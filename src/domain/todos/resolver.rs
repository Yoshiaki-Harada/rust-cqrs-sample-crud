use std::sync::Arc;
use crate::domain::todos::store as todo_store;

pub struct TodoResolver {
    todo_store: todo_store::PostgresDb,
}

impl Default for TodoResolver {
    fn default() -> Self {
        TodoResolver {
            todo_store: todo_store::init_connection()
        }
    }
}

impl TodoResolver {
    pub fn get_pool(&self) -> todo_store::PostgresDb {
        self.todo_store.clone()
    }
}

impl TodoResolver {
    pub(in crate::domain::todos) fn todo_store(&self) -> impl todo_store::TodoStore {
        todo_store::TodoDb {}
    }
}