pub mod htmx;
pub mod layout;
pub mod pages;
pub mod partials;

pub use htmx::{is_htmx_request, render_page_or_fragment};
