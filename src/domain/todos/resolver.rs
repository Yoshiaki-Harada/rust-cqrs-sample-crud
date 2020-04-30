use std::sync::Arc;
use crate::domain::todos::store as todo_store;

pub struct TodoResolver {
    todo_store: Arc<todo_store::InMemoryStore>,
}

impl Default for TodoResolver {
    fn default() -> Self {
        TodoResolver {
            todo_store: Arc::new(todo_store::in_memory_store())
        }
    }
}

impl TodoResolver {
    pub(in crate::domain::todos) fn todo_store(&self) -> impl todo_store::TodoStore {
        // Arcのcloneなので複数スレッドから参照できる
        self.todo_store.clone()
    }
}