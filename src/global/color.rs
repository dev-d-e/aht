use super::*;
use std::sync::OnceLock;

//default background color
static BG_COLOR: OnceLock<Color> = OnceLock::new();

pub(super) fn set_default_bg_color(color: Color) {
    let _ = BG_COLOR.set(color);
}

pub(crate) fn default_bg_color() -> &'static Color {
    BG_COLOR.get_or_init(|| Color::from_rgb(255, 255, 255))
}

//default border color
static BORDER_COLOR: OnceLock<Color> = OnceLock::new();

pub(super) fn set_default_border_color(color: Color) {
    let _ = BORDER_COLOR.set(color);
}

pub(crate) fn default_border_color() -> &'static Color {
    BORDER_COLOR.get_or_init(|| Color::from_rgb(0, 0, 0))
}

//default font color
static FONT_COLOR: OnceLock<Color> = OnceLock::new();

pub(super) fn set_default_font_color(color: Color) {
    let _ = FONT_COLOR.set(color);
}

pub(crate) fn default_font_color() -> &'static Color {
    FONT_COLOR.get_or_init(|| Color::from_rgb(0, 0, 0))
}

//default surface color
static SURFACE_COLOR: OnceLock<Color> = OnceLock::new();

pub(super) fn set_default_surface_color(color: Color) {
    let _ = SURFACE_COLOR.set(color);
}

pub(crate) fn default_surface_color() -> &'static Color {
    SURFACE_COLOR.get_or_init(|| Color::from_rgb(200, 200, 200))
}

//default button color
static BUTTON_COLOR: OnceLock<Color> = OnceLock::new();

pub(super) fn set_default_button_color(color: Color) {
    let _ = BUTTON_COLOR.set(color);
}

pub(crate) fn default_button_color() -> &'static Color {
    BUTTON_COLOR.get_or_init(|| Color::from_rgb(100, 200, 100))
}

//default button color
static BUTTON2_COLOR: OnceLock<Color> = OnceLock::new();

pub(super) fn set_default_button2_color(color: Color) {
    let _ = BUTTON2_COLOR.set(color);
}

pub(crate) fn default_button2_color() -> &'static Color {
    BUTTON2_COLOR.get_or_init(|| Color::from_rgb(100, 100, 200))
}

//default blank color
static BLANK_COLOR: OnceLock<Color> = OnceLock::new();

pub(super) fn set_default_blank_color(color: Color) {
    let _ = BLANK_COLOR.set(color);
}

pub(crate) fn default_blank_color() -> &'static Color {
    BLANK_COLOR.get_or_init(|| Color::from_rgb(255, 255, 255))
}

//default cursor color
static CURSOR_COLOR: OnceLock<Color> = OnceLock::new();

pub(super) fn set_default_cursor_color(color: Color) {
    let _ = CURSOR_COLOR.set(color);
}

pub(crate) fn default_cursor_color() -> &'static Color {
    CURSOR_COLOR.get_or_init(|| Color::from_rgb(100, 100, 100))
}

//default scroll bar color
static SCROLL_BAR_COLOR: OnceLock<Color> = OnceLock::new();

pub(super) fn set_default_scroll_bar_color(color: Color) {
    let _ = SCROLL_BAR_COLOR.set(color);
}

pub(crate) fn default_scroll_bar_color() -> &'static Color {
    SCROLL_BAR_COLOR.get_or_init(|| Color::from_rgb(200, 200, 200))
}

//default scroll bar color
static SCROLL_BAR2_COLOR: OnceLock<Color> = OnceLock::new();

pub(super) fn set_default_scroll_bar2_color(color: Color) {
    let _ = SCROLL_BAR2_COLOR.set(color);
}

pub(crate) fn default_scroll_bar2_color() -> &'static Color {
    SCROLL_BAR2_COLOR.get_or_init(|| Color::from_rgb(0, 100, 100))
}
