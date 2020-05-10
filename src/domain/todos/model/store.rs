use std::sync::Arc;
use crate::domain::todos::{Todo, TodoData, NewTodo};
use auto_impl::auto_impl;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::pg::PgConnection;
use std::{collections::{hash_map::Entry, HashMap}, sync::RwLock, vec::IntoIter, env};
use crate::schema;
use diesel::prelude::*;
use std::borrow::Borrow;
use std::error::Error;
use diesel::insert_into;
use dotenv::dotenv;

// connectionを受け取る
#[auto_impl(&)]
pub(in crate::domain) trait TodoStore {
    fn get_todo(&self, id: i32, conn: &PgConnection) -> Todo;
    fn set_todo(&self, todo: Todo, conn: &PgConnection);
}

#[auto_impl(&, Arc)]
pub(in crate::domain) trait TodoStoreFilter {
    fn Filter<F>(&self, predicate: F) -> Iter
        where
            F: Fn(&TodoData) -> bool;
}

pub(in crate::domain) type Iter = IntoIter<TodoData>;


// impl TodoStoreFilter for PostgresDb {
//     fn Filter<F>(&self, predicate: F) -> Iter
//         where
//             F: Fn(&TodoData) -> bool,
//     {
//         let todos: Vec<_> = self
//             .read()
//             .unwrap()
//             .values()
//             .filter(|p| predicate(*p))
//             .cloned()
//             .collect();
//         todos.into_iter()
//     }
// }
//

pub struct TodoDb {}

impl TodoStore for TodoDb {
    fn get_todo(&self, todoId: i32, conn: &PgConnection) -> Todo {
        use crate::schema::todo::dsl::*;
        let result = todo.find(todoId).first(conn);
        Todo::from_data(result.unwrap())
    }

    fn set_todo(&self, new_todo: Todo, conn: &PgConnection) {
        use crate::schema::todo::dsl::*;
        let data = new_todo.into_data();
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
