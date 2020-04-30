use crate::domain::todos::*;
use rocket_contrib::json::Json;
use crate::domain::Resolver;
use rocket::State;
use serde::Serialize;

#[derive(Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[get("/<id>")]
pub fn get(id: i32, resolver: State<Resolver>) -> Json<Todo> {
    let query = resolver.get_todo_query();
    let todo = query.get_todo(GetTodo{id:id}).into_data();
    Json(Todo {
        id: todo.id,
        title: todo.title,
        description: todo.description,
        done: todo.done
    })
}