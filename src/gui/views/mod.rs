pub mod label;
pub mod button;
pub mod edit;

use gui::themes::{Typeface, ViewState};
use gui::traits::{View, WeakElement};
use gui::types::Rect;
use std::rc::Rc;
use speedy2d::font::FormattedTextBlock;
use std::collections::HashMap;
use gui::events::UiEvent;
use gui::ui::UI;
pub use self::label::Label;
pub use self::button::Button;
pub use self::edit::Edit;

pub struct FieldsTexted {
    pub main: FieldsMain,
    pub text: String,
    pub text_size: f32,
    pub cached_text: Option<Rc<FormattedTextBlock>>,
    pub listeners: HashMap<UiEvent, Box<dyn FnMut(&mut UI, &dyn View) -> bool>>
}

pub struct FieldsMain {
    pub rect: Rect<i32>,
    pub padding: Borders,
    pub id: String,
    pub state: ViewState,
    pub pressed: bool,
    pub parent: Option<WeakElement>,
    pub typeface: Option<Typeface>
}

impl FieldsMain {
    pub fn with_rect(rect: Rect<i32>) -> Self {
        FieldsMain {
            rect,
            padding: Borders::default(),
            id: String::new(),
            state: ViewState::Idle,
            pressed: false,
            parent: None,
            typeface: None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Borders {
    pub top: i32,
    pub left: i32,
    pub right: i32,
    pub bottom: i32
}

impl Borders {
    pub fn new(top: i32, left: i32, right: i32, bottom: i32) -> Self {
        Self { top, left, right, bottom }
    }

    pub fn with_padding(padding: i32) -> Self {
        Self { top: padding, left: padding, right: padding, bottom: padding }
    }
}

impl Default for Borders {
    fn default() -> Self {
        Self::with_padding(4)
    }
}
