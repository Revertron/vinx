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

pub struct ViewTextable {
    pub rect: Rect<i32>,
    pub id: String,
    pub state: ViewState,
    pub pressed: bool,
    pub parent: Option<WeakElement>,
    pub typeface: Option<Typeface>,
    pub text: String,
    pub text_size: f32,
    pub cached_text: Option<Rc<FormattedTextBlock>>,
    pub listeners: HashMap<UiEvent, Box<dyn FnMut(&mut UI, &dyn View) -> bool>>
}

pub struct ViewFields {
    pub rect: Rect<i32>,
    pub id: String,
    pub parent: Option<WeakElement>,
    pub typeface: Option<Typeface>
}
