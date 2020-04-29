use crate::domain::todos::model::{Todo, TodoData};
use auto_impl::auto_impl;
use std::{
    collections::{hash_map::Entry, HashMap},
    sync::RwLock,
    vec::IntoIter,
};

#[auto_impl(&, Arc)]
pub(in crate::domain) trait TodoStore {
    fn get_todo(&self, id: i32) -> Todo;
    fn set_todo(&self, todo: Todo);
}

#[auto_impl(&, Arc)]
pub(in crate::domain) trait TodoStoreFilter {
    fn Filter<F>(&self, predicate: F) -> Iter
    where
        F: Fn(&TodoData) -> bool;
}

pub(in crate::domain) type Iter = IntoIter<TodoData>;

pub(in crate::domain) type InMemoryStore = RwLock<HashMap<i32, TodoData>>;

impl TodoStore for InMemoryStore {
    fn get_todo(&self, id: i32) -> Todo {
        let todos = self.read().unwrap();
        if let Some(data) = todos.get(&id) {
            Todo::from_data(data.clone())
        } else {
            panic!("{} is not found", id)
        }
    }

    fn set_todo(&self, todo: Todo) {
        let data = todo.into_data();
        let mut todos = self.write().unwrap();
        todos.insert(data.id, data);
    }
}

impl TodoStoreFilter for InMemoryStore {
    fn Filter<F>(&self, predicate: F) -> Iter
    where
        F: Fn(&TodoData) -> bool,
    {
        let todos: Vec<_> = self
            .read()
            .unwrap()
            .values()
            .filter(|p| predicate(*p))
            .cloned()
            .collect();
        todos.into_iter()
    }
}

pub(in crate::domain::todos) fn in_memory_store() -> InMemoryStore {
    RwLock::new(HashMap::new())
}