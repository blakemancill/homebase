use maud::{Markup, html};

pub(crate) fn render_login_page() -> Markup {
    html! {
        form method="post" action="/login" {
            div .card {
                div .card-content {
                    p {
                        label for="username" { "Username" }
                        input .input name="username" type="text" id="username" autofocus {};
                    }
                    br;
                    p {
                        label for="password" { "Password" }
                        input .input name="password" type="password" id="password" {};
                    }
                    button .button.is-primary.mt-3 {
                        "Submit"
                    }
                }
            }
        }
    }
}
