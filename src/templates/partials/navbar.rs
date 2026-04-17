use maud::{Markup, html};

pub fn render_navbar(current_path: &str) -> Markup {
    html! {
        nav .navbar #sidebar hx-swap-oob="true" {
            div .container {
                div .navbar-menu {
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
        a .navbar-item.has-text-white.is-active[is_active]
            href=(href)
            hx-get=(href)
            hx-target="#main-content"
            hx-push-url="true"
        { (label) }
    }
}