use crate::features::transactions::render_csv_upload_section;
use maud::{Markup, html};

pub fn render_index() -> Markup {
    html! {
        (render_csv_upload_section())
    }
}
