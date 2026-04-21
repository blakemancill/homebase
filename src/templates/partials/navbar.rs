use maud::{Markup, PreEscaped, html};

pub fn render_navbar(current_path: &str) -> Markup {
    html! {
        style {
            r#"
                @media (max-width: 768px) {
                    .is-hidden-narrow { display: none; }
                }
            "#
        }
        button .is-hidden-tablet.m-3 type="button"
            _="on click toggle .is-hidden-narrow on #sidebar"
        {
            (PreEscaped(
                r#"
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none"
                    viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                    width="24" height="24">
                    <path stroke-linecap="round" stroke-linejoin="round"
                    d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" /></svg>
                "#
            ))
        }

        aside #sidebar .menu.is-hidden-narrow {
            div .menu {
                 ul .menu-list {
                    (nav_link("/", "Home", current_path))
                    (nav_link("/dashboard", "Budget", current_path))
                }
            }
        }
    }
}

fn nav_link(href: &str, label: &str, current_path: &str) -> Markup {
    let is_active = current_path == href;
    html! {
        li {
            a .is-active[is_active] href=(href) { (label) }
        }
    }
}
