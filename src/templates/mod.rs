pub mod layout;
pub mod partials;
pub mod pages;
pub mod htmx;

pub use htmx::{render_page_or_fragment, is_htmx_request};