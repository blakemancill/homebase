use maud::{Markup, html};

pub fn render_sidebar() -> Markup {
    html! {
        // Alpine component: stores current path and updates on HTMX navigation / popstate
        div x-data="{ currentPath: window.location.pathname }"
            x-init="
                document.addEventListener('htmx:afterRequest', () => { currentPath = window.location.pathname });
                window.addEventListener('popstate', () => { currentPath = window.location.pathname });
            " {
            nav .navbar {
                div .container {
                    div .navbar-menu {
                        a .navbar-item.has-text-white
                            x-bind:class="{ 'is-active': currentPath === '/' }"
                            href="/"
                            hx-get="/"
                            hx-target="#main-content"
                            hx-push-url="true"
                            x-on:click="currentPath = '/'"
                        {
                            "Home"
                        }
                        a .navbar-item.has-text-white
                            x-bind:class="{ 'is-active': currentPath === '/dashboard' }"
                            href="/dashboard"
                            hx-get="/dashboard"
                            hx-target="#main-content"
                            hx-push-url="true"
                            x-on:click="currentPath = '/dashboard'"
                        {
                            "Dashboard"
                        }
                    }
                }
            }
        }
    }
}
