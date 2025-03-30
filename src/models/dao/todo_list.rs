use redis::Commands;

use crate::models::todo::{TodoItem, TodoList};

pub async fn get_todolist<S: ToString>(
    r: &mut redis::Connection,
    todolist_name: S,
) -> Option<TodoList> {
    match r.lrange::<String, Vec<String>>(todolist_name.to_string(), 0, -1) {
        Ok(strings) => Some(TodoList {
            name: todolist_name.to_string(),
            items: strings
                .iter()
                .map(|body| TodoItem { body: body.clone() })
                .collect(),
        }),
        Err(_) => None,
    }
}
