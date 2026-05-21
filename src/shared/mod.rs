//! Cross-cutting helpers shared across features.
//!
//! Where [`features`](crate::features) modules are vertical slices of the app,
//! these are the horizontal pieces every slice can use.
//!
//! - [`base`]: the page shell (`base_layout`) wrapping content in HTML, `<head>`, navbar.
//! - [`currency`]: money helpers that allow parse/format of dollars, stored as integer pennies.
//! - [`navbar`]: the shared navigation bar markup.

pub mod base;
pub mod currency;
pub mod navbar;
