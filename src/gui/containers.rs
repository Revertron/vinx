use std::cell::RefCell;
use std::rc::Rc;

use speedy2d::dimen::Vector2;
use speedy2d::window::MouseButton;

use gui::themes::Theme;
use gui::traits::{Element, WeakElement};
use gui::types::{Point, Rect, rect};
use gui::ui::UI;
use gui::views::FieldsMain;
use themes::{FontStyle, Typeface};
use traits::{Container, View};

pub struct Frame {
    state: RefCell<FieldsMain>,
    views: Vec<Element>,
}

impl Frame {
    pub fn new(rect: Rect<i32>) -> Frame {
        Frame {
            state: RefCell::new(FieldsMain::with_rect(rect)),
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

    fn layout(&mut self, rect: &Rect<i32>, typeface: &Typeface, scale: f64) {
        println!("Laying out for size {}x{}", rect.width(), rect.height());
        for v in self.views.iter() {
            let typeface = match self.state.borrow().typeface.clone() {
                None => typeface.clone(),
                Some(t) => t
            };
            let mut v = v.try_borrow_mut().unwrap();
            v.layout(rect, &typeface, scale);
        }
    }

    fn paint(&self, origin: Point<i32>, theme: &mut dyn Theme) {
        let mut rect = self.state.borrow().rect;
        rect.move_by(origin);
        theme.set_clip(rect);
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
        Frame::new(rect)
    }
}