use serde::Deserialize;

pub struct TodoItem {
    pub body: String,
}

#[derive(Default)]
pub struct TodoList {
    pub name: String,
    pub items: Vec<TodoItem>,
}


#[derive(Deserialize, Debug)]
pub struct NewTodoItemForm {
    pub todoitem_body: String,
}