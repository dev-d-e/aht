/*!
A module for global configuration.
*/

mod color;
mod font;

use crate::markup::*;
use crate::utils::*;
pub(crate) use color::*;
pub(crate) use font::*;
use skia_safe::Font;

///InitialFont
pub struct InitialFont(Font, Option<Color>);

impl InitialFont {
    pub fn new(s: &str, color: Option<Color>) -> Option<Self> {
        get_font(s).map(|font| Self(font, color))
    }

    pub fn initialize(self) {
        set_default_font(self.0);
        if let Some(color) = self.1 {
            set_default_font_color(color);
        }
        info!("initial font");
    }
}
