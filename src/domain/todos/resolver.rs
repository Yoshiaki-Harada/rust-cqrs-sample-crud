use std::sync::{Arc, RwLock};
use crate::domain::todos::store as todo_store;
use crate::domain::todos::model::store::TodoDb;

pub struct TodoResolver {
    todo_store: Arc<todo_store::TodoDb>,
    pool: todo_store::PostgresDb,
}

impl Default for TodoResolver {
    fn default() -> Self {
        TodoResolver {
            todo_store: Arc::new(todo_store::init_todo_store()),
            pool: todo_store::init_connection(),
        }
    }
}

impl TodoResolver {
    pub fn get_pool(&self) -> todo_store::PostgresDb {
        self.pool.clone()
    }
}

impl TodoResolver {
    pub(in crate::domain::todos) fn todo_store(&self) -> impl todo_store::TodoStore {
        self.todo_store.clone()
    }
}