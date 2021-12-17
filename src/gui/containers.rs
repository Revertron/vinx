use std::cell::RefCell;
use std::rc::Rc;

use speedy2d::dimen::Vector2;
use speedy2d::window::MouseButton;

use themes::{FontStyle, Theme, Typeface, ViewState};
use traits::{Container, Element, View, WeakElement};
use types;
use types::{Point, Rect, rect};
use ui::UI;
use views::{Dimension, Direction, FieldsMain};

pub struct Frame {
    state: RefCell<FieldsMain>,
    direction: Direction,
    views: Vec<Element>,
}

impl Frame {
    pub fn new(rect: Rect<i32>, width: Dimension, height: Dimension) -> Frame {
        Frame {
            state: RefCell::new(FieldsMain::with_rect(rect, width, height)),
            direction: Direction::default(),
            views: Vec::new(),
        }
    }

    fn set_font(&mut self, font_name: &str) {
        let typeface = match self.state.borrow_mut().typeface.take() {
            None => Typeface { font_name: font_name.to_owned(), font_style: FontStyle::Regular },
            Some(mut t) => {
                t.font_name = font_name.to_owned();
                t
            }
        };
        self.state.borrow_mut().typeface = Some(typeface)
    }

    fn set_font_style(&mut self, style: &str) {
        let font_style = FontStyle::from(style);
        let typeface = match self.state.borrow_mut().typeface.take() {
            None => Typeface { font_name: String::new(), font_style },
            Some(t) => Typeface { font_name: t.font_name, font_style },
        };
        self.state.borrow_mut().typeface = Some(typeface)
    }

    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}

impl Container for Frame {
    fn add_view(&mut self, view: Element) {
        self.views.push(view);
    }

    fn get_view(&self, id: &str) -> Option<Element> {
        println!("Searching View with id {}", &id);
        if let Some(found) = self.views.iter().find(|&view| view.borrow().get_id() == id) {
            return Some(Rc::clone(found));
        }

        for v in self.views.iter() {
            if let Some(found) = v.borrow().as_container() {
                return found.get_view(id);
            }
        }
        None
    }

    fn get_view_count(&self) -> usize {
        self.views.len()
    }
}

impl View for Frame {
    fn set_any(&mut self, name: &str, value: &str) {
        match name {
            "left" => { self.set_x(value.parse().unwrap()) }
            "top" => { self.set_y(value.parse().unwrap()) }
            "width" => { self.set_width(value.parse().unwrap()) }
            "height" => { self.set_height(value.parse().unwrap()) }
            "direction" => { self.set_direction(value.parse().unwrap()) }
            "id" => { self.set_id(value) }
            "font" => { self.set_font(value) }
            "font_style" => { self.set_font_style(value) }
            &_ => {}
        }
    }

    fn set_parent(&self, parent: Option<WeakElement>) {
        self.state.borrow_mut().parent = parent;
    }

    fn get_parent(&self) -> Option<Element> {
        match &self.state.borrow().parent {
            None => { None }
            Some(weak) => {
                match weak.upgrade() {
                    None => { None }
                    Some(rc) => { Some(rc) }
                }
            }
        }
    }

    fn layout_content(&mut self, x: i32, y: i32, width: i32, height: i32, typeface: &Typeface, scale: f64) -> Rect<i32> {
        //println!("Laying out for size {}x{}", rect.width(), rect.height());
        let (new_width, new_height) = self.calculate_size(width, height, scale);

        let padding = self.get_padding(scale);
        let mut xx = padding.left;
        let mut yy = padding.top;
        let typeface = match self.state.borrow().typeface.clone() {
            None => typeface.clone(),
            Some(t) => t
        };
        for v in self.views.iter() {
            let mut v = v.try_borrow_mut().unwrap();
            v.layout_content(xx, yy, new_width - xx - padding.right, new_height - yy - padding.bottom, &typeface, scale);
            // Get maximum occupied area
            let view_size = v.calculate_full_size(scale);
            match self.direction {
                Direction::Horizontal => xx = xx + view_size.0,
                Direction::Vertical => yy = yy + view_size.1
            }
        }

        let (w, h) = self.calculate_full_size(scale);
        let (width, height) = {
            let mut state = self.state.borrow_mut();
            let mut ww = w;
            let mut hh = h;
            match &state.width {
                Dimension::Min => ww = w,
                Dimension::Max => ww = new_width,
                Dimension::Dip(dip) => ww = *dip as i32,
                Dimension::Percent(p) => ww = (width as f32 * p / 100f32).round() as i32
            }
            match &state.height {
                Dimension::Min => hh = h,
                Dimension::Max => hh = new_height,
                Dimension::Dip(dip) => hh = *dip as i32,
                Dimension::Percent(p) => hh = (height as f32 * p / 100f32).round() as i32
            }
            (ww, hh)
        };
        let rect = rect((x, y), (x + width, y + height));
        self.set_rect(rect.clone());
        println!("Frame {} sizes: {:?}", self.get_id(), &rect);
        rect
    }

    fn fits_in_rect(&self, width: i32, height: i32, scale: f64) -> bool {
        let size = self.calculate_full_size(scale);
        size.0 <= width && size.1 <= height
    }

    fn paint(&self, origin: Point<i32>, theme: &mut dyn Theme) {
        let mut rect = self.state.borrow().rect;
        rect.move_by(origin);
        theme.set_clip(rect);
        theme.draw_panel_back(rect, ViewState::Idle);
        theme.draw_panel_body(rect, ViewState::Idle);
        for v in self.views.iter() {
            let v = v.try_borrow().unwrap();
            v.paint(self.state.borrow().rect.min + origin, theme);
        }
    }

    fn get_rect(&self) -> Rect<i32> {
        self.state.borrow().rect
    }

    fn set_rect(&mut self, rect: Rect<i32>) {
        self.state.borrow_mut().rect = rect;
    }

    fn get_bounds(&self) -> (Dimension, Dimension) {
        let state = self.state.borrow();
        (state.width, state.height)
    }

    fn get_content_size(&self) -> (i32, i32) {
        let mut rect = rect((-1, -1), (0, 0));
        for v in self.views.iter() {
            let mut v = v.borrow();
            // Get maximum occupied area
            let view_rect = v.get_rect();
            if rect.min.x == -1 || view_rect.min.x < rect.min.x {
                rect.min.x = view_rect.min.x;
            }
            if rect.min.y == -1 || view_rect.min.y < rect.min.y {
                rect.min.y = view_rect.min.y;
            }
            if view_rect.max.x > rect.max.x {
                rect.max.x = view_rect.max.x;
            }
            if view_rect.max.y > rect.max.y {
                rect.max.y = view_rect.max.y;
            }
        }
        (rect.width(), rect.height())
    }

    fn set_width(&mut self, width: Dimension) {
        self.state.borrow_mut().width = width;
    }

    fn set_height(&mut self, height: Dimension) {
        self.state.borrow_mut().height = height;
    }

    fn set_id(&mut self, id: &str) {
        self.state.borrow_mut().id = id.to_owned();
    }

    fn get_id(&self) -> String {
        self.state.borrow().id.clone()
    }

    fn as_container(&self) -> Option<&dyn Container> {
        Some(self as &dyn Container)
    }

    fn as_container_mut(&mut self) -> Option<&mut dyn Container> {
        Some(self as &mut dyn Container)
    }

    fn onclick(&mut self, _func: Box<dyn FnMut(&mut UI, &dyn View) -> bool>) {
        // No op
    }

    fn click(&self, _ui: &mut UI) -> bool {
        // No op
        false
    }

    fn on_mouse_move(&self, ui: &mut UI, position: Vector2<i32>) -> bool {
        let position = (position.x - self.state.borrow().rect.min.x, position.y - self.state.borrow().rect.min.y);
        let mut processed = false;
        for v in self.views.iter().rev() {
            processed |= v.borrow().on_mouse_move(ui, Vector2::from(position));
        }
        processed
    }

    fn on_mouse_button_down(&self, ui: &mut UI, position: Vector2<i32>, button: MouseButton) -> bool {
        println!("Mouse down in {}", &self.state.borrow().id);
        let position = (position.x - self.state.borrow().rect.min.x, position.y - self.state.borrow().rect.min.y);
        for v in self.views.iter().rev() {
            if v.borrow().on_mouse_button_down(ui, Vector2::from(position), button) {
                return true;
            }
        }
        false
    }

    fn on_mouse_button_up(&self, ui: &mut UI, position: Vector2<i32>, button: MouseButton) -> bool {
        let position = (position.x - self.state.borrow().rect.min.x, position.y - self.state.borrow().rect.min.y);
        for v in self.views.iter().rev() {
            if v.borrow().on_mouse_button_up(ui, Vector2::from(position), button) {
                return true;
            }
        }
        false
    }
}

impl Default for Frame {
    fn default() -> Self {
        let rect = rect((0, 0), (400, 300));
        Frame::new(rect, Dimension::Max, Dimension::Min)
    }
}