mod color;
mod font;
mod net;

pub(crate) use color::*;
pub(crate) use font::*;
use net::*;
use skia_safe::{Color, Font};

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
    }
}

///InitialNet
pub struct InitialNet;

impl InitialNet {
    pub fn initialize(o: impl Net) {
        set_default_net(o);
    }
}
