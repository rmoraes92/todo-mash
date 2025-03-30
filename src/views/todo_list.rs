use maud::{Markup, html};

use crate::models::todo::TodoList;

pub fn view(todo_list: &TodoList) -> Markup {
    html!(
        div id="todolist" class="card shadow" {
            div class="card-header bg-primary text-white" {
                h1 class="h4 mb-0" {
                    (todo_list.name)
                }
            }
            div class="card-body" {
                @for item in &todo_list.items {
                    div class="todo-item d-flex align-items-center border-bottom py-2" {
                        div class="form-check flex-grow-1" {
                            input class="form-check-input" type="checkbox" id="todo-1";
                            label class="form-check-label" for="todo-1" { (item.body) }
                        }
                    }
                }
            }
        }
    )
}
