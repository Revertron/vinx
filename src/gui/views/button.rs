use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::HashMap;

use speedy2d::dimen::Vector2;
use speedy2d::font::{TextAlignment, TextLayout, TextOptions};
use speedy2d::window::MouseButton;

use assets::get_font;
use events::UiEvent;
use gui::themes::{FontStyle, Theme, Typeface, ViewState};
use gui::traits::{Container, Element, View, WeakElement};
use gui::types::{Point, Rect, rect};
use gui::ui::UI;
use gui::views::{Borders, Dimension};
use views::{FieldsMain, FieldsTexted};
use crate::gui::views::{BUTTON_MIN_HEIGHT, BUTTON_MIN_WIDTH};

pub struct Button {
    state: RefCell<FieldsTexted>
}

#[allow(dead_code)]
impl Button {
    pub fn new(rect: Rect<i32>, text: &str, text_size: f32) -> Button {
        Button {
            state: RefCell::new(FieldsTexted {
                main: FieldsMain::with_rect(rect, Dimension::Min, Dimension::Min),
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

    fn layout_text(&self, max_width: i32, scale: f64) {
        if max_width <= 0 {
            self.state.borrow_mut().cached_text = None;
            return;
        }
        let typeface = self.state.borrow().main.typeface.clone();
        if let Some(typeface) = typeface {
            if let Some(font) = get_font(&typeface.font_name, &typeface.font_style.to_string()) {
                let options = TextOptions::new().with_wrap_to_width(max_width as f32, TextAlignment::Left);
                let size = self.state.borrow().text_size * scale as f32;
                let text = font.layout_text(&self.state.borrow().text, size, options);
                self.state.borrow_mut().cached_text = Some(text);
            }
        }
    }
}

impl View for Button {
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
        println!("{} for width {}", self.get_id(), width);
        let typeface = self.get_typeface(typeface);
        self.state.borrow_mut().main.typeface = Some(typeface);
        self.state.borrow_mut().main.scale = scale;
        // TODO use padding
        let (new_width, new_height) = self.calculate_size(width, height, scale);
        self.layout_text(new_width, scale);
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
        theme.push_clip();
        theme.clip_rect(rect);
        theme.draw_button_back(rect, state.main.state);
        theme.draw_button_body(rect, state.main.state);
        // TODO use padding
        if let Some(text) = &state.cached_text {
            let x = (self.get_rect_width() as f32 - text.width()) / 2f32;
            let y = (self.get_rect_height() as f32 - text.height()) / 2f32;
            theme.draw_text((rect.min.x as f32 + x).round(), (rect.min.y as f32 + y).round(), text);
        }
        theme.pop_clip();
    }

    fn get_rect(&self) -> Rect<i32> {
        self.state.borrow().main.rect
    }

    fn set_rect(&mut self, rect: Rect<i32>) {
        self.state.borrow_mut().main.rect = rect;
    }

    fn get_padding(&self, scale: f64) -> Borders {
        self.state.borrow().main.padding.scaled(scale)
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
                let width = max(text.width().ceil() as i32, BUTTON_MIN_WIDTH);
                let height = max(text.height().ceil() as i32, BUTTON_MIN_HEIGHT);
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
        let listener = self.state.borrow_mut().listeners.remove(&UiEvent::Click);
        if let Some(mut click) = listener {
            let result = click(ui, self as &dyn View);
            self.state.borrow_mut().listeners.insert(UiEvent::Click, click);
            return result;
        }
        false
    }

    fn on_mouse_move(&self, ui: &mut UI, position: Vector2<i32>) -> bool {
        let hit = self.state.borrow().main.rect.hit((position.x, position.y));
        let old_state = self.state.borrow_mut().main.state;
        self.state.borrow_mut().main.state = if hit {
            if matches!(self.state.borrow().main.state, ViewState::Idle) && self.state.borrow().main.pressed {
                ViewState::Pressed
            } else {
                match self.state.borrow().main.state {
                    ViewState::Idle => ViewState::Hovered,
                    ViewState::Hovered => ViewState::Hovered,
                    ViewState::Pressed => ViewState::Pressed,
                    ViewState::Disabled => ViewState::Disabled
                }
            }
        } else {
            ViewState::Idle
        };
        self.state.borrow_mut().main.state != old_state
    }

    fn on_mouse_button_down(&self, ui: &mut UI, position: Vector2<i32>, button: MouseButton) -> bool {
        if self.state.borrow().main.rect.hit((position.x, position.y)) && matches!(button, MouseButton::Left) {
            let mut state = self.state.borrow_mut();
            state.main.pressed = true;
            state.main.state = ViewState::Pressed;
            return true;
        }
        false
    }

    fn on_mouse_button_up(&self, ui: &mut UI, position: Vector2<i32>, button: MouseButton) -> bool {
        let hit = self.state.borrow().main.rect.hit((position.x, position.y));
        if self.state.borrow().main.pressed && hit {
            println!("Doing click!");
            self.click(ui);
            let mut state = self.state.borrow_mut();
            state.main.pressed = false;
            state.main.state = ViewState::Hovered;
            return true;
        }
        if self.state.borrow().main.pressed && !hit && matches!(button, MouseButton::Left) {
            println!("Cancelled click");
            let mut state = self.state.borrow_mut();
            state.main.pressed = false;
            state.main.state = ViewState::Idle;
            return true;
        }
        false
    }
}

impl Default for Button {
    fn default() -> Self {
        let rect = rect((0, 0), (60, 24));
        Button::new(rect, "", 24_f32)
    }
}
