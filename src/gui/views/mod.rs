pub mod label;
pub mod button;
pub mod edit;
pub mod checkbox;
pub mod list;
pub mod listview;

use gui::themes::{Typeface, ViewState};
use gui::traits::{View, WeakElement};
use gui::types::Rect;
use std::rc::Rc;
use speedy2d::font::FormattedTextBlock;
use std::collections::HashMap;
use std::str::FromStr;
use gui::common::random_string;
use gui::events::EventType;
use gui::ui::UI;
use super::styles::selector::{MainSelector, FontSelector};
pub use self::label::Label;
pub use self::button::Button;
pub use self::edit::Edit;
pub use self::checkbox::CheckBox;
pub use self::list::List;

pub const BUTTON_MIN_WIDTH: i32 = 80;
pub const BUTTON_MIN_HEIGHT: i32 = 24;

/// Stores all main fields of elements.
pub struct FieldsMain {
    pub width: Dimension,
    pub height: Dimension,
    pub rect: Rect<i32>,
    pub padding: Borders,
    pub margin: Borders,
    pub scale: f64,
    pub id: String,
    pub state: ViewState,
    pub break_line: bool,
    pub background: MainSelector,
    pub foreground: MainSelector,
    pub parent: Option<WeakElement>,
    pub typeface: Option<Typeface>
}

impl FieldsMain {
    /// Convenient method to create main fields.
    /// Most of these values will be changed in `layout()` methods.
    pub fn with_rect(rect: Rect<i32>, width: Dimension, height: Dimension) -> Self {
        FieldsMain {
            width,
            height,
            rect,
            padding: Borders::default(),
            margin: Borders::default(),
            scale: 1.0,
            id: random_string(16),
            state: ViewState::default(),
            break_line: false,
            background: MainSelector::new(),
            foreground: MainSelector::new(),
            parent: None,
            typeface: None
        }
    }
}

/// Stores main fields (properties) of elements, plus fields for text.
pub struct FieldsTexted {
    pub main: FieldsMain,
    pub text: String,
    pub text_size: f32,
    pub line_height: f32,
    pub single_line: bool,
    pub cached_text: Option<Rc<FormattedTextBlock>>,
    pub font: FontSelector,
    pub listeners: HashMap<EventType, Box<dyn FnMut(&mut UI, &dyn View) -> bool>>
}

/// Represents padding (inner spaces) or margin (outer spaces) of any element.
#[derive(Clone, Copy, Debug)]
pub struct Borders {
    pub top: i32,
    pub left: i32,
    pub right: i32,
    pub bottom: i32
}

#[allow(unused)]
impl Borders {
    pub fn new(top: i32, left: i32, right: i32, bottom: i32) -> Self {
        Self { top, left, right, bottom }
    }

    pub fn with_padding(padding: i32) -> Self {
        Self { top: padding, left: padding, right: padding, bottom: padding }
    }

    pub fn set_all(&mut self, padding: i32) {
        self.top = padding;
        self.left = padding;
        self.right = padding;
        self.bottom = padding;
    }

    pub fn scaled(&self, scale: f64) -> Self {
        Self {
            top: (self.top as f64 * scale).ceil() as i32,
            left: (self.left as f64 * scale).ceil() as i32,
            right: (self.right as f64 * scale).ceil() as i32,
            bottom: (self.bottom as f64 * scale).ceil() as i32
        }
    }
}

impl Default for Borders {
    fn default() -> Self {
        Self::with_padding(0)
    }
}

/// Elements width or height. They can fill up all space in some direction (Max),
/// or just enough space to wrap its content (Min), or set concrete size
/// in terms of device independent pixels (Dip, they will be scaled),
/// or some fraction of available area (Percent).
#[derive(Clone, Copy, Debug)]
pub enum Dimension {
    Min,
    Max,
    Dip(u32),
    Percent(f32)
}

impl FromStr for Dimension {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "max" => Dimension::Max,
            "min" => Dimension::Min,
            &_ => {
                if s.ends_with("%") {
                    let float = match s[0..s.len()-1].parse::<f32>() {
                        Ok(float) => float,
                        Err(e) => {
                            println!("Error parsing {}, {}", s, e);
                            0f32
                        }
                    };
                    Dimension::Percent(float)
                } else {
                    let int = match s[0..s.len()].parse::<u32>() {
                        Ok(int) => int,
                        Err(e) => {
                            println!("Error parsing {}, {}", s, e);
                            0u32
                        }
                    };
                    Dimension::Dip(int)
                }
            }
        };
        Ok(result)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Horizontal
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "vertical" => Direction::Vertical,
            &_ => Direction::Horizontal
        };
        Ok(result)
    }
}