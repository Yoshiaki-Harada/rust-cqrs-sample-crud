use crate::domain::todos::*;
use rocket_contrib::json::Json;
use crate::domain::Resolver;
use rocket::State;
use serde::{Serialize, Deserialize};

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
    let todo = query.get_todo(GetTodo { id: id }).new_data();
    Json(Todo {
        id: todo.id,
        title: todo.title,
        description: todo.description,
        done: todo.done,
    })
}

#[get("/?<title>")]
pub fn get_by_title(title: String, resolver: State<Resolver>) -> Json<Vec<Todo>> {
    let query = resolver.get_todo_by_title_query();
    let data: Vec<TodoData> = query.get_todo_by_title(GetTodoByTitle { title }).into_iter().map(|t| t.into_data()).rev().collect();
    Json(data.into_iter().map(|d| Todo { id: d.id, title: d.title, description: d.description, done: d.done }).rev().collect())
}

#[derive(Deserialize)]
pub struct UpdateInputTodo {
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[put("/<id>", data = "<todo>")]
pub fn update(id: i32, todo: Json<UpdateInputTodo>, resolver: State<Resolver>) {
    let mut command = resolver.create_todo_command();
    command.create_todo(CreateTodo {
        id: id,
        title: todo.0.title,
        description: todo.0.description,
        done: todo.0.done,
    })
}

#[derive(Deserialize)]
pub struct CreateInputTodo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[post("/", data = "<todo>")]
pub fn create(todo: Json<CreateInputTodo>, resolver: State<Resolver>) {
    let mut command = resolver.create_todo_command();
    command.create_todo(CreateTodo {
        id: todo.0.id,
        title: todo.0.title,
        description: todo.0.description,
        done: todo.0.done,
    })
}