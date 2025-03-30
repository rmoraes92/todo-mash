use axum::{
    Extension, Form,
    response::{Html, IntoResponse},
};
use redis::{Client, Commands};

use crate::{
    models::{dao::todo_list::get_todolist, todo::{NewTodoItemForm, TodoList}},
    views,
};

pub async fn controller(
    Extension(conn): Extension<Client>,
    Form(payload): Form<NewTodoItemForm>,
) -> impl IntoResponse {
    let mut r: redis::Connection = conn.get_connection().unwrap();

    r
        .rpush::<String, String, usize>(
            "todo".to_owned(),
            payload.todoitem_body,
        )
        .unwrap();

    // .rpush(Str!("todo"), payload.todoitem_body);
    let todo_list = match get_todolist(&mut r, "todo").await {
        Some(t) => t,
        None => TodoList::default(),
    };
    Html(views::todo_list::view(&todo_list).into_string())
}
