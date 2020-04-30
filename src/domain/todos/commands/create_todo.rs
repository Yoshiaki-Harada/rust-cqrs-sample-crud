use crate::domain::Resolver;
use auto_impl::auto_impl;
use crate::domain::todos::{Todo, TodoStore};

#[derive(Clone)]
pub struct CreateTodo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[auto_impl(FnMut)]
pub trait CreateTodoCommand {
    fn create_todo(&mut self, command: CreateTodo);
}

pub(in crate::domain) fn create_todo_command(store: impl TodoStore) -> impl CreateTodoCommand{
    move |command: CreateTodo| {
        store.set_todo(Todo::new(command.id, command.title, command.description, command.done));
        println!("create todo {}", command.id);
    }
}

impl Resolver {
    pub fn create_todo_command(&self) -> impl CreateTodoCommand {
        let store = self.todos().todo_store();
        create_todo_command(store)
    }
}