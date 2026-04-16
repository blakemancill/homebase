use maud::{html, Markup};

pub fn render_sidebar() -> Markup {
    html! {
        nav .navbar.bg-body-tertiary {
            div .container-fluid {
                // Topbar content
                button .navbar-toggler data-bs-toggle="offcanvas" data-bs-target="#offcanvasNavbar" {
                    span .navbar-toggler-icon {}
                }
                h6 { "Personal Finance Tracker v0.1" }

                // Offcanvas navigation sidebar
                div .offcanvas.offcanvas-start tabindex="-1" #offcanvasNavbar {
                    div .offcanvas-header {
                        h5 .offcanvas-title { "Navigation" }
                        button .btn-close data-bs-dismiss="offcanvas" {}
                    }
                    div .offcanvas-body {
                        // Navigation Links
                        ul .navbar-nav.justify-content-end.flex-grow-1.pe-3 {
                            li .nav-item {
                                a .nav-link href="/" hx-get="/" hx-target="#main-content" hx-push-url="true" { "Home" }
                                a .nav-link href="/dashboard" hx-get="/dashboard" hx-target="#main-content" hx-push-url="true" { "Dashboard" }
                            }
                        }
                    }
                }
            }
        }
    }
}