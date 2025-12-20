/*!
It's another hypertext format to describe GUI for native application.

The components build on plane rectangular coordinates.

# Examples
```
use aht::Page;

Page::parse("<aht>
    <head>
        <title></title>
    </head>
    <body>
        <inp name="" value="" readonly required>input</inp>
        <button href="" class=a>button</button>
        <area id="b"></area>
    </body>
    <style>
    body {
        column:[100,100];
        row:[100,100],2;
    }
    inp {
        ordinal:0;
        height:100;
    }
    .a {
        position=100,100;
    }
    .id = b {
        width:1000;
        height:100;
        column:2;
        row:1;
    }
    </style>
    <script>
    </script>
 </aht>");

 Page::parse_s("<aht>
    < head>
        < title>
    <:body>
        < inp name="" value="" readonly required>input
        <button href="" class=a>button
        <area class="" id="b">
    <:style>
    body {
        column:[100,100];
        row:[100,100],2;
    }
    inp {
        ordinal:0;
        height:100;
    }
    .a {
        position=100,100;
    }
    .id = b {
        width:1000;
        height:100;
        column:2;
        row:1;
    }
    <script>");
```

# Cargo Features

| Feature            | Description                                      |
|--------------------|--------------------------------------------------|
| `vulkan`           | Enables the Vulkan backend.                      |
| `js`               | Enables javascript.                              |
| `window`           | Enables winit window.                            |
*/

#![allow(dead_code, unused_imports, unused_variables)]

#[macro_use]
mod macros;
mod content;
pub mod error;
pub mod global;
mod gpu;
mod imagesound;
pub mod markup;
mod metadata;
mod page;
mod screen;
mod script;
mod style;
mod utils;

pub use page::*;
pub use screen::*;
