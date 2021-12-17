use std::cell::RefCell;
use std::cmp::max;
use std::collections::HashMap;

use speedy2d::font::{TextAlignment, TextLayout, TextOptions};

use assets::get_font;
use events::UiEvent;
use themes::{FontStyle, Theme, Typeface, ViewState};
use traits::{Container, Element, View, WeakElement};
use types::{Point, Rect, rect};
use ui::UI;
use views::{BUTTON_MIN_HEIGHT, BUTTON_MIN_WIDTH, Dimension, FieldsMain, FieldsTexted};

pub struct Edit {
    state: RefCell<FieldsTexted>
}

#[allow(dead_code)]
impl Edit {
    pub fn new(rect: Rect<i32>, text: &str, text_size: f32) -> Edit {
        Edit {
            state: RefCell::new(FieldsTexted {
                main: FieldsMain::with_rect(rect, Dimension::Max, Dimension::Min),
                text: text.to_owned(),
                text_size,
                cached_text: None,
                listeners: HashMap::new()
            })
        }
    }

    pub fn set_text(&self, text: &str) {
        {
            let mut state = self.state.borrow_mut();
            state.text.clear();
            state.text.push_str(text);
            state.cached_text = None;
        }
        let scale = self.state.borrow().main.scale;
        self.layout_text(self.get_rect_width(), scale);
    }

    fn get_typeface(&self, parent_typeface: &Typeface) -> Typeface {
        match &self.state.borrow().main.typeface {
            None => parent_typeface.clone(),
            Some(t) => {
                if t.font_name.is_empty() {
                    let mut parent = parent_typeface.clone();
                    parent.font_style = t.font_style.clone();
                    parent
                } else {
                    t.clone()
                }
            }
        }
    }

    fn set_font(&self, font_name: &str) {
        let typeface = match self.state.borrow_mut().main.typeface.take() {
            None => Typeface { font_name: font_name.to_owned(), font_style: FontStyle::Regular },
            Some(mut t) => {
                t.font_name = font_name.to_owned();
                t
            }
        };
        self.state.borrow_mut().main.typeface = Some(typeface);
    }

    fn set_font_style(&self, style: &str) {
        let font_style = FontStyle::from(style);
        let typeface = match self.state.borrow_mut().main.typeface.take() {
            None => Typeface { font_name: String::new(), font_style },
            Some(t) => Typeface { font_name: t.font_name, font_style },
        };
        self.state.borrow_mut().main.typeface = Some(typeface)
    }

    #[allow(unused_variables)]
    fn layout_text(&self, width: i32, scale: f64) {
        let typeface = self.state.borrow().main.typeface.clone();
        if let Some(typeface) = typeface {
            if let Some(font) = get_font(&typeface.font_name, &typeface.font_style.to_string()) {
                let options = TextOptions::new();
                let text = font.layout_text(&self.state.borrow().text, self.state.borrow().text_size, options);
                self.state.borrow_mut().cached_text = Some(text);
            }
        }
    }
}

impl View for Edit {
    fn set_any(&mut self, name: &str, value: &str) {
        match name {
            "left" => { self.set_x(value.parse().unwrap()) }
            "top" => { self.set_y(value.parse().unwrap()) }
            "width" => { self.set_width(value.parse().unwrap()) }
            "height" => { self.set_height(value.parse().unwrap()) }
            "id" => { self.set_id(value) }
            "text" => { self.set_text(value) }
            "font" => { self.set_font(value) }
            "font_style" => { self.set_font_style(value) }
            &_ => {}
        }
    }

    fn set_parent(&self, parent: Option<WeakElement>) {
        self.state.borrow_mut().main.parent = parent;
    }

    fn get_parent(&self) -> Option<Element> {
        match &self.state.borrow().main.parent {
            None => { None }
            Some(weak) => {
                match weak.upgrade() {
                    None => { None }
                    Some(parent) => { Some(parent) }
                }
            }
        }
    }

    fn layout_content(&mut self, x: i32, y: i32, width: i32, height: i32, typeface: &Typeface, scale: f64) -> Rect<i32> {
        if self.state.borrow().cached_text.is_some() {
            // TODO check if area changed
            return self.get_rect();
        }

        let typeface = self.get_typeface(typeface);
        self.state.borrow_mut().main.typeface = Some(typeface);
        self.state.borrow_mut().main.scale = scale;
        self.layout_text(width, scale);
        let (width, height) = self.calculate_full_size(scale);
        let rect = rect((x, y), (x + width, y + height));
        self.set_rect(rect.clone());
        rect
    }

    fn fits_in_rect(&self, width: i32, height: i32, scale: f64) -> bool {
        let state = self.state.borrow();
        match &state.cached_text {
            Some(text) => text.width() <= width as f32 && text.height() <= height as f32,
            None => width <= BUTTON_MIN_WIDTH && height <= BUTTON_MIN_HEIGHT
        }
    }

    fn paint(&self, origin: Point<i32>, theme: &mut dyn Theme) {
        let state = self.state.borrow();
        let mut rect = state.main.rect;
        rect.move_by(origin);
        // Drawing the back and frame
        theme.set_clip(rect);
        theme.draw_edit_back(rect, state.main.state);
        theme.draw_edit_body(rect, state.main.state);
        // Drawing the text
        let padding = state.main.padding.scaled(state.main.scale);
        rect.shrink_by(padding.top, padding.left, padding.right, padding.bottom);
        theme.set_clip(rect);
        if let Some(text) = &state.cached_text {
            let y = (self.get_rect_height() as f32 - text.height() - padding.top as f32 - padding.bottom as f32) / 2f32;
            theme.draw_text((rect.min.x as f32 + padding.left as f32).round(), (rect.min.y as f32 + y).round(), text);
        }
    }

    fn get_rect(&self) -> Rect<i32> {
        self.state.borrow().main.rect
    }

    fn set_rect(&mut self, rect: Rect<i32>) {
        self.state.borrow_mut().main.rect = rect;
    }

    fn get_bounds(&self) -> (Dimension, Dimension) {
        let state = self.state.borrow();
        (state.main.width, state.main.height)
    }

    fn get_content_size(&self) -> (i32, i32) {
        let state = self.state.borrow();
        match &state.cached_text {
            None => (BUTTON_MIN_WIDTH, BUTTON_MIN_HEIGHT),
            Some(text) => {
                let width = max(text.width().round() as i32, BUTTON_MIN_WIDTH);
                let height = max(text.height().round() as i32, BUTTON_MIN_HEIGHT);
                (width, height)
            }
        }
    }

    fn set_width(&mut self, width: Dimension) {
        self.state.borrow_mut().main.width = width;
    }

    fn set_height(&mut self, height: Dimension) {
        self.state.borrow_mut().main.height = height;
    }

    fn set_id(&mut self, id: &str) {
        self.state.borrow_mut().main.id = id.to_owned();
    }

    fn get_id(&self) -> String {
        self.state.borrow().main.id.clone()
    }

    fn as_container(&self) -> Option<&dyn Container> {
        None
    }

    fn as_container_mut(&mut self) -> Option<&mut dyn Container> {
        None
    }

    fn onclick(&mut self, func: Box<dyn FnMut(&mut UI, &dyn View) -> bool>) {
        self.state.borrow_mut().listeners.insert(UiEvent::Click, func);
    }

    fn click(&self, ui: &mut UI) -> bool {
        if let Some(mut click) = self.state.borrow_mut().listeners.remove(&UiEvent::Click) {
            let result = click(ui, self as &dyn View);
            self.state.borrow_mut().listeners.insert(UiEvent::Click, click);
            return result;
        }
        false
    }
}

impl Default for Edit {
    fn default() -> Self {
        let rect = rect((0, 0), (60, 24));
        Edit::new(rect, "", 48_f32)
    }
}
