use maud::{html, Markup};

pub fn view<S: ToString>(title: S) -> Markup {
    html!(
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title {
                (title.to_string())
            }
            script src="https://unpkg.com/htmx.org@1.9.12" {}
            link href="https://cdnjs.cloudflare.com/ajax/libs/bootstrap/5.3.2/css/bootstrap.min.css" rel="stylesheet";
            link href="https://cdnjs.cloudflare.com/ajax/libs/bootstrap-icons/1.11.1/font/bootstrap-icons.min.css" rel="stylesheet";
        }
    )
}