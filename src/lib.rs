/*!
It's another hypertext format to describe GUI for native application.

The components build on plane rectangular coordinates.

# Examples
```
use aht::Page;

aht::markup::Element::parse_one("<aht>
    <head>
        <title></title>
    </head>
    <body column='[100,100]' row='[100,100],2'>
        <inp name='' value='' readonly required>input</inp>
        <button href=''>button</button>
        <area class='' id='' width='1000' height='100' column='2' row=''></area>
    </body>
    <style>
    </style>
    <script>
    </script>
 </aht>");
Page::parse("<aht>
    <head>
        <title></title>
    </head>
    <body column='[100,100]' row='[100,100],2'>
        <inp name='' value='' readonly required>input</inp>
        <button href=''>button</button>
        <area class='' id='' width='1000' height='100' column='2' row=''></area>
    </body>
    <style>
    </style>
    <script>
    </script>
 </aht>");
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
