//!It's another hypertext format to describe GUI for native application.
//!
//!```
//!use aht::markup::{Element, Page};
//!
//!Element::from_str(&s);
//!
//!Page::from_str(&s);
//!```
//!

#![allow(dead_code)]

#[macro_use]
mod macros;
pub mod content;
pub mod global;
mod grid;
mod head;
pub mod markup;
mod metadata;
pub mod parts;
mod script;
mod style;
mod utils;
