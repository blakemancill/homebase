use crate::shared::currency::format_pennies;
use maud::{Markup, html};

pub fn render_index(net_worth: i64) -> Markup {
    html! {
        div .container {
            div .is-flex.is-justify-content-center {
                (render_net_worth_card(net_worth))
            }
        }
    }
}

pub fn render_net_worth_card(net_worth: i64) -> Markup {
    html! {
        div .box .has-text-centered {
            p .heading.is-uppercase { "Net Worth" }
            p .title.is-3.has-text-weight-bold { (format_pennies(net_worth)) }
        }
    }
}
