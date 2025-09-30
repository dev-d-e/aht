//!It's another hypertext format to describe GUI for native application.
//!
//!The components build on plane rectangular coordinates.
//!
//!```
//!use aht::markup::{Element, Page};
//!
//!Element::from_str(&s);
//!
//!Page::from_str(&s);
//!```
//!

#![allow(dead_code, unused_imports, unused_variables)]

#[macro_use]
mod macros;
pub mod content;
pub mod global;
mod gpu;
mod grid;
mod head;
mod imagesound;
pub mod markup;
mod metadata;
pub mod parts;
mod script;
mod style;
mod utils;
