use maud::{html, Markup};

pub(crate) fn render_account_dashboard() -> Markup {
    html! {
        p { "This is the account dashboard." }
    }
}