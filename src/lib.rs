//!It's another hypertext format to describe GUI for native application.
//!
//!```
//!use aht::markup::{Page, TypeEntity};
//!
//!TypeEntity::from_str(&s);
//!
//!Page::from_str(&s);
//!```
//!

#![allow(dead_code)]

#[macro_use]
mod macros;
pub mod content;
pub mod grid;
pub mod head;
pub mod markup;
pub mod metadata;
pub mod parts;
mod script;
mod style;
mod utils;

#[cfg(test)]
mod tests {

    use crate::markup::{Page, TypeEntity};

    #[test]
    fn test() {
        let s = "<aht>
        <head lang=en>
            <title>1</title>
        </head>
        <body column=\"\" row=\"2\">
            <inp name=\"\" value=\"\" readonly required>input</inp>
            <button href=\"\" async=>button</button>
            <area class=\"\" id=\"\" width=\"1000\" height=\"100\" column=2 row=\"\"></area>
        </body>
        <style>
        </style>
        <script>
        </script>
     </aht>";
        if let Some(e) = TypeEntity::from_str(&s) {
            println!("{:?}", e);
        }
        if let Some(e) = Page::from_str(&s) {
            println!("{:?}", e);
        }
    }
}
