use maud::{Markup, html};

pub fn render_navbar(current_path: &str) -> Markup {
    html! {
        aside #sidebar .menu hx-swap-oob="true" {
            div .menu {
                 ul .menu-list {
                    (nav_link("/", "Home", current_path))
                    (nav_link("/dashboard", "Dashboard", current_path))
                }
            }
        }
    }
}

fn nav_link(href: &str, label: &str, current_path: &str) -> Markup {
    let is_active = current_path == href;
    html! {
        li {
            a .is-active[is_active]
                href=(href)
                hx-get=(href)
                hx-target="#main-content"
                hx-push-url="true"
            { (label) }
        }
    }
}