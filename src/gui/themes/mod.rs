mod classic;
mod utils;

use std::rc::Rc;
use speedy2d::font::FormattedTextBlock;
use gui::styles::selector::MainSelector;
pub use super::themes::classic::Classic;
use gui::types::Rect;

pub trait Theme {
    fn clear_screen(&mut self);
    fn typeface() -> Typeface where Self: Sized;
    fn get_back_color(&self, state: ViewState, selector: &MainSelector) -> u32;
    fn get_text_color(&self, state: ViewState, selector: &MainSelector) -> u32;
    fn set_clip(&mut self, rect: Rect<i32>);
    fn clip_rect(&mut self, rect: Rect<i32>) -> Rect<i32>;
    fn push_clip(&mut self);
    fn pop_clip(&mut self);
    fn draw_button_back(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_button_body(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_button_text(&mut self, rect: Rect<i32>, state: ViewState, size: usize, text: &str);
    fn draw_edit_back(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_edit_body(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_edit_caret(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_checkbox_back(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_checkbox_body(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_list_back(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_list_body(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_panel_back(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_panel_body(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_text(&mut self, x: f32, y: f32, color: u32, text: &Rc<FormattedTextBlock>);
    fn draw_rect(&mut self, rect: Rect<i32>, color: u32);
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum FontStyle {
    Regular,
    Bold,
    Italic,
    BoldItalic
}

impl ToString for FontStyle {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl From<&str> for FontStyle {
    fn from(s: &str) -> Self {
        match s {
            "Bold" => FontStyle::Bold,
            "Italic" => FontStyle::Italic,
            "BoldItalic" => FontStyle::BoldItalic,
            &_ => FontStyle::Regular
        }
    }
}

impl From<String> for FontStyle {
    fn from(s: String) -> Self {
        FontStyle::from(s.as_str())
    }
}

#[derive(Clone)]
pub struct Typeface {
    pub font_name: String,
    pub font_style: FontStyle
}

impl Default for Typeface {
    fn default() -> Self {
        Typeface { font_name: String::from("NotoSans"), font_style: FontStyle::Regular }
    }
}

#[allow(unused)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct ViewState {
    pub enabled: bool,
    pub focusable: bool,
    pub focused: bool,
    pub hovered: bool,
    pub pressed: bool,
    pub checked: bool
}

#[allow(unused)]
impl ViewState {
    pub fn no_focus() -> Self {
        ViewState {
            enabled: true,
            focusable: false,
            focused: false,
            hovered: false,
            pressed: false,
            checked: false
        }
    }
}

impl Default for ViewState {
    fn default() -> Self {
        ViewState {
            enabled: true,
            focusable: true,
            focused: false,
            hovered: false,
            pressed: false,
            checked: false
        }
    }
}