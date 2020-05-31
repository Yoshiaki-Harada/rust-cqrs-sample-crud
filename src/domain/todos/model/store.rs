use std::sync::Arc;
use crate::domain::todos::{Todo, TodoData, NewTodo};
use auto_impl::auto_impl;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::pg::PgConnection;
use std::{collections::{hash_map::Entry, HashMap}, sync::RwLock, vec::IntoIter, env};
use crate::schema;
use diesel::prelude::*;
use diesel::insert_into;
use dotenv::dotenv;

// connectionを受け取る
#[auto_impl(&, Arc)]
pub(in crate::domain) trait TodoStore {
    fn get_todo(&self, id: i32, conn: &PgConnection) -> Todo;
    fn get_todo_by_title(&self, title: &str, conn: &PgConnection) -> Vec<Todo>;
    fn set_todo(&self, todo: Todo, conn: &PgConnection);
}

#[auto_impl(&, Arc)]
pub(in crate::domain) trait TodoStoreFilter {
    fn Filter<F>(&self, predicate: F) -> Iter
        where
            F: Fn(&TodoData) -> bool;
}

pub(in crate::domain) type Iter = IntoIter<TodoData>;

pub struct TodoDb {}

pub(in crate::domain::todos) fn init_todo_store() -> TodoDb {
    TodoDb {}
}

impl TodoStore for TodoDb {
    fn get_todo(&self, todoId: i32, conn: &PgConnection) -> Todo {
        use crate::schema::todo::dsl::*;
        let result = todo.find(todoId).first(conn);
        Todo::from_data(result.unwrap())
    }

    fn get_todo_by_title(&self, todoTitle: &str, conn: &PgConnection) -> Vec<Todo> {
        use crate::schema::todo::dsl::*;
        let result = todo.filter(title.eq(todoTitle)).load::<TodoData>(conn);
        result.unwrap().into_iter().map(|data| Todo::from_data(data)).rev().collect()
    }

    fn set_todo(&self, new_todo: Todo, conn: &PgConnection) {
        use crate::schema::todo::dsl::*;
        let data = new_todo.new_data();
        insert_into(todo).values(&data).get_result::<TodoData>(conn).expect("error");
    }
}

pub type PostgresDb = Pool<ConnectionManager<PgConnection>>;

pub(in crate::domain::todos) fn init_connection() -> PostgresDb {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL  must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed  to create pool")
}
