use maud::{Markup, html};

use crate::{
    models::todo::TodoList,
    views::{footer, head, todo_list},
};

pub fn view<S: ToString>(title: S, todo_list: &TodoList) -> Markup {
    html! {
        (maud::DOCTYPE)
        html {
            (head::view(title))
            body {
                div class="container py-5" {
                    div class="row justify-content-center" {
                        div class="col-md-8" {

                            div class="input-group" {
                                input id="todoitem_body" name="todoitem_body" class="form-control" placeholder="Add a new task...";
                                button hx-post="/todos/new" hx-target="#todolist" hx-include="#todoitem_body" type="button" class="btn btn-primary" {
                                    i class="bi bi-plus-lg" {} "Add"
                                }
                            }
                            br;
                            (todo_list::view(todo_list))
                        }
                    }
                }
                (footer::view())
            }
        }
    }
}
