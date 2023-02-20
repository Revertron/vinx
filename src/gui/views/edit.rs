use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::time::Instant;
use speedy2d::dimen::Vector2;
use speedy2d::font::{TextAlignment, TextLayout, TextOptions};
use speedy2d::window::{KeyScancode, ModifiersState, MouseButton, VirtualKeyCode};

use assets::get_font;
use events::UiEvent;
use gui;
use gui::common::{delete_char, insert_char};
use gui::views::Borders;
use styles::selector::FontSelector;
use themes::{FontStyle, Theme, Typeface, ViewState};
use traits::{Container, Element, View, WeakElement};
use types::{Point, Rect, rect};
use ui::UI;
use views::{BUTTON_MIN_HEIGHT, BUTTON_MIN_WIDTH, Dimension, FieldsMain, FieldsTexted};

pub struct Edit {
    state: RefCell<FieldsTexted>,
    scroll_x: RefCell<i32>,
    caret_pos: RefCell<usize>,
    caret_rect: RefCell<Rect<i32>>,
    caret_time: RefCell<Instant>
}

#[allow(dead_code)]
impl Edit {
    pub fn new(rect: Rect<i32>, text: &str, text_size: f32) -> Edit {
        let mut fields = FieldsTexted {
            main: FieldsMain::with_rect(rect, Dimension::Max, Dimension::Min),
            text: text.to_owned(),
            text_size,
            line_height: 0f32,
            single_line: true,
            cached_text: None,
            foreground: FontSelector::new(),
            listeners: HashMap::new()
        };
        fields.main.padding = Borders::with_padding(4);
        Edit {
            state: RefCell::new(fields),
            scroll_x: RefCell::new(0),
            caret_pos: RefCell::new(0),
            caret_rect: RefCell::new(gui::types::rect((0, 0), (0, 0))),
            caret_time: RefCell::new(Instant::now())
        }
    }

    pub fn set_text(&self, text: &str) {
        {
            let mut state = self.state.borrow_mut();
            state.text.clear();
            state.text.push_str(text);
            state.cached_text = None;
            let chars_count = state.text.chars().count();
            if *self.caret_pos.borrow() > chars_count {
                *self.caret_pos.borrow_mut() = chars_count;
                self.caret_rect.borrow_mut().clear();
            }
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
        if self.state.borrow().text.len() == 0 {
            self.state.borrow_mut().cached_text = None;
            return;
        }
        let typeface = self.state.borrow().main.typeface.clone();
        if let Some(typeface) = typeface {
            if let Some(font) = get_font(&typeface.font_name, &typeface.font_style.to_string()) {
                let options = TextOptions::new();
                let text = font.layout_text(&self.state.borrow().text, self.state.borrow().text_size, options);
                self.state.borrow_mut().cached_text = Some(text);
            }
        }
    }

    fn update_caret_rect(&self, scale: f64) {
        let padding = self.get_padding(scale);
        let mut rect = self.caret_rect.borrow().clone();
        let caret_pos = self.caret_pos.borrow();
        let my_rect = self.state.borrow().main.rect;
        if let Some(text) = &self.state.borrow().cached_text {
            for line in text.iter_lines() {
                rect.min.y = my_rect.min.y + padding.top + 2;
                rect.max.y = my_rect.max.y - padding.bottom - 2;
                if *caret_pos == 0 {
                    rect.min.x = my_rect.min.x + padding.left;
                    rect.max.x = rect.min.x + (1f64 * scale) as i32;
                } else {
                    let mut count = 0;
                    for glyph in line.iter_glyphs() {
                        if count == (*caret_pos - 1) {
                            let glyph_right = glyph.position_x().ceil() as i32 + glyph.advance_width().ceil() as i32;
                            rect.min.x = my_rect.min.x + padding.left + glyph_right;
                            rect.max.x = rect.min.x + (1f64 * scale) as i32;
                            break;
                        }
                        count += 1;
                    }
                }
            }
        } else {
            rect.min.x = my_rect.min.x + padding.left;
            rect.max.x = rect.min.x + (1f64 * scale) as i32;
            rect.min.y = my_rect.min.y + padding.top + 2;
            rect.max.y = my_rect.max.y - padding.bottom - 2;
        }
        *self.caret_rect.borrow_mut() = rect;
        *self.caret_time.borrow_mut() = Instant::now();
    }

    fn get_caret_rect(&self, scale: f64) -> Rect<i32> {
        let mut rect = self.caret_rect.borrow().clone();
        if rect.width() != 0 && rect.height() != 0 {
            return rect;
        }
        self.update_caret_rect(scale);
        self.caret_rect.borrow().clone()
    }

    fn update_scroll(&self) {
        let scale = self.state.borrow().main.scale;
        let shift = (60f64 * scale).round() as i32;
        let my_rect = self.state.borrow().main.rect;
        let rect = self.get_caret_rect(scale);
        let padding = self.get_padding(scale);
        let cur_scroll_x = *self.scroll_x.borrow();
        if rect.max.x + cur_scroll_x > my_rect.max.x {
            let (width, _height) = self.get_content_size();
            let view_width = my_rect.width() - padding.left - padding.right;
            let min_x = view_width - width;
            *self.scroll_x.borrow_mut() = min_x.max(cur_scroll_x - shift);
        } else if rect.min.x + cur_scroll_x < my_rect.min.x + padding.left {
            let max_x = 0;
            *self.scroll_x.borrow_mut() = max_x.min(cur_scroll_x + shift);
        }
    }

    fn get_line_height(&self) -> f32 {
        if self.state.borrow().line_height != 0f32 {
            return self.state.borrow().line_height;
        }

        let typeface = self.state.borrow().main.typeface.clone();
        if let Some(typeface) = typeface {
            if let Some(font) = get_font(&typeface.font_name, &typeface.font_style.to_string()) {
                let options = TextOptions::new();
                let text = font.layout_text("W", self.state.borrow().text_size, options);
                self.state.borrow_mut().line_height = text.height();
            }
        }
        self.state.borrow_mut().line_height
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
            "break" => { self.state.borrow_mut().main.break_line = value.parse().unwrap_or(false) }
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
        if self.state.borrow().cached_text.is_none() {
            let typeface = self.get_typeface(typeface);
            self.state.borrow_mut().main.typeface = Some(typeface);
            self.state.borrow_mut().main.scale = scale;
            self.layout_text(width, scale);
        }
        let (new_width, new_height) = self.calculate_size(width, height, scale);
        let (w, h) = self.calculate_full_size(scale);
        let (width, height) = {
            let mut state = self.state.borrow_mut();
            let mut ww = w;
            let mut hh = h;
            match &state.main.width {
                Dimension::Min => ww = w,
                Dimension::Max => ww = new_width,
                Dimension::Dip(dip) => ww = *dip as i32,
                Dimension::Percent(p) => ww = (width as f32 * p / 100f32).round() as i32
            }
            match &state.main.height {
                Dimension::Min => hh = h,
                Dimension::Max => hh = new_height,
                Dimension::Dip(dip) => hh = *dip as i32,
                Dimension::Percent(p) => hh = (height as f32 * p / 100f32).round() as i32
            }
            (ww, hh)
        };
        let rect = rect((x, y), (x + width, y + height));
        self.set_rect(rect);
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
        self.update_scroll();
        let state = self.state.borrow();
        let scroll_x = *self.scroll_x.borrow();
        let mut rect = state.main.rect;
        rect.move_by(origin);
        //println!("Drawing Edit {} in rect: {:?}, starting rect {:?}", self.get_id(), &rect, &state.main.rect);
        // Drawing the back and frame
        theme.push_clip();
        theme.clip_rect(rect);
        theme.draw_edit_back(rect, state.main.state);
        theme.draw_edit_body(rect, state.main.state);
        theme.pop_clip();
        // Drawing the text
        let padding = state.main.padding.scaled(state.main.scale);
        rect.shrink_by(padding.top, padding.left, padding.right, padding.bottom);
        theme.push_clip();
        theme.clip_rect(rect);
        if let Some(text) = &state.cached_text {
            let y = (rect.height() as f32 - text.height()) / 2f32;
            theme.draw_text((rect.min.x as f32 + scroll_x as f32).round(), (rect.min.y as f32 + y).round(), text);
        }
        theme.pop_clip();
        let elapsed = self.caret_time.borrow().elapsed().as_millis();
        if elapsed < 500 || elapsed % 1000 < 500 {
            let mut caret_rect = self.get_caret_rect(state.main.scale);
            caret_rect.move_by((scroll_x, 0));
            theme.draw_edit_caret(caret_rect, state.main.state);
        }
    }

    fn get_state(&self) -> Option<ViewState> {
        Some(self.state.borrow().main.state)
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
        let line_height = self.get_line_height().round() as i32;
        let state = self.state.borrow();
        match &state.cached_text {
            None => {
                (BUTTON_MIN_WIDTH, line_height)
            },
            Some(text) => {
                let width = max(text.width().round() as i32, BUTTON_MIN_WIDTH);
                let height = max(text.height().round() as i32, line_height);
                (width, height)
            }
        }
    }

    fn is_focused(&self) -> bool {
        self.state.borrow().main.state.focused
    }

    fn is_break(&self) -> bool {
        self.state.borrow().main.break_line
    }

    fn set_focused(&self, focused: bool) {
        self.state.borrow_mut().main.state.focused = focused;
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

    fn on_mouse_button_down(&self, _ui: &mut UI, position: Vector2<i32>, button: MouseButton) -> bool {
        if self.state.borrow().main.rect.hit((position.x, position.y)) {
            let mut state = self.state.borrow_mut();
            if matches!(button, MouseButton::Left) {
                state.main.state.pressed = true;
            }
            state.main.state.focused = true;
            return true;
        }
        false
    }

    fn on_key_down(&self, _ui: &mut UI, virtual_key_code: Option<VirtualKeyCode>, _scancode: KeyScancode, _state: ModifiersState) -> bool {
        if let Some(code) = virtual_key_code {
            match code {
                VirtualKeyCode::Left => {
                    let mut caret_pos = self.caret_pos.borrow_mut();
                    if *caret_pos > 0 {
                        *caret_pos -= 1;
                        self.caret_rect.borrow_mut().clear();
                    }
                    return true;
                }
                VirtualKeyCode::Right => {
                    let mut caret_pos = self.caret_pos.borrow_mut();
                    if *caret_pos < self.state.borrow().text.chars().count() {
                        *caret_pos += 1;
                        self.caret_rect.borrow_mut().clear();
                    }
                    return true;
                }
                VirtualKeyCode::Home => {
                    *self.caret_pos.borrow_mut() = 0;
                    self.caret_rect.borrow_mut().clear();
                    return true;
                }
                VirtualKeyCode::End => {
                    let new_pos = self.state.borrow().text.chars().count();
                    *self.caret_pos.borrow_mut() = new_pos;
                    self.caret_rect.borrow_mut().clear();
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    fn on_key_up(&self, _ui: &mut UI, _virtual_key_code: Option<VirtualKeyCode>, _scancode: KeyScancode, _state: ModifiersState) -> bool {
        //TODO
        false
    }

    fn on_key_char(&self, _ui: &mut UI, ch: char, _state: ModifiersState) -> bool {
        let pos = *self.caret_pos.borrow();
        println!("on_key_char with {}, pos {}", ch, pos);
        if ch.is_alphanumeric() || ch >= ' ' || ch == '\u{8}' || ch == '\u{7f}' {
            match ch {
                '\u{8}' => {
                    if pos > 0 {
                        let new_text = delete_char(&self.state.borrow().text, pos - 1);
                        self.state.borrow_mut().text = new_text;
                        *self.caret_pos.borrow_mut() -= 1;
                    }
                }
                '\u{7f}' => {
                    let new_text = delete_char(&self.state.borrow().text, pos);
                    self.state.borrow_mut().text = new_text;
                }
                _ => {
                    let new_text = insert_char(&self.state.borrow().text, pos, ch);
                    self.state.borrow_mut().text = new_text;
                    *self.caret_pos.borrow_mut() += 1;
                }
            }
            self.state.borrow_mut().cached_text = None;
            self.caret_rect.borrow_mut().clear();
            let scale = self.state.borrow().main.scale;
            self.layout_text(self.get_rect_width(), scale);
            return true;
        }

        true
    }

    fn on_key_mod_changed(&self, _ui: &mut UI, _state: ModifiersState) -> bool {
        //TODO
        false
    }
}

impl Default for Edit {
    fn default() -> Self {
        let rect = rect((0, 0), (60, 24));
        Edit::new(rect, "", 48_f32)
    }
}
