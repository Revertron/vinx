mod classic;

use std::rc::Rc;
use speedy2d::font::FormattedTextBlock;
pub use themes::classic::Classic;
use gui::types::Rect;

pub trait Theme {
    fn clear_screen(&mut self);
    fn typeface() -> Typeface where Self: Sized;
    fn set_clip(&mut self, rect: Rect<i32>);
    fn draw_button_back(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_button_body(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_button_text(&mut self, rect: Rect<i32>, state: ViewState, size: usize, text: &str);
    fn draw_edit_back(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_edit_body(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_panel_back(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_panel_body(&mut self, rect: Rect<i32>, state: ViewState);
    fn draw_text(&mut self, x: f32, y: f32, text: &Rc<FormattedTextBlock>);
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
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ViewState {
    Idle,
    Hovered,
    Pressed,
    Disabled
}