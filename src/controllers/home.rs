use axum::{
    response::{Html, IntoResponse}, Extension}
;
use redis::Client;

use crate::{
    models::{dao::todo_list::get_todolist, todo::TodoList},
    views,
};

pub async fn controller(
    Extension(client): Extension<Client>,
) -> impl IntoResponse {
    let mut r: redis::Connection = client.get_connection().unwrap();
    let todo_list = match get_todolist(&mut r, "todo").await {
        Some(t) => t,
        None => TodoList::default(),
    };
    Html(views::index::view("Johnny Boy", &todo_list).into_string())
}
