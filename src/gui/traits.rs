use std::cell::RefCell;
use std::rc::{Rc, Weak};
use downcast_rs::Downcast;
use speedy2d::dimen::Vector2;
use speedy2d::window::{KeyScancode, ModifiersState, MouseButton, VirtualKeyCode};
use gui::ui::UI;
use gui::themes::Theme;
use gui::types::{Rect, Point};
use themes::Typeface;

pub type Element = Rc<RefCell<dyn View>>;
pub type WeakElement = Weak<RefCell<dyn View>>;

//pub type Parent = Rc<RefCell<dyn Container>>;
//pub type WeakParent = Weak<RefCell<dyn Container>>;

pub trait View: Downcast {
    fn set_any(&mut self, name: &str, value: &str);
    fn set_parent(&self, parent: Option<WeakElement>);
    fn get_parent(&self) -> Option<Element>;
    #[allow(unused)]
    fn layout(&mut self, rect: &Rect<i32>, typeface: &Typeface, scale: f64) {}
    fn paint(&self, origin: Point<i32>, theme: &mut dyn Theme);
    fn get_rect(&self) -> Rect<i32>;
    fn set_rect(&mut self, rect: Rect<i32>);
    fn get_x(&self) -> i32 { self.get_rect().min.x }
    fn get_y(&self) -> i32 { self.get_rect().min.y }
    fn get_width(&self) -> i32 { self.get_rect().width() }
    fn get_height(&self) -> i32 { self.get_rect().height() }
    fn set_x(&mut self, x: i32) {
        let mut rect = self.get_rect();
        rect.move_to((x, rect.min.y));
        self.set_rect(rect);
    }
    fn set_y(&mut self, y: i32) {
        let mut rect = self.get_rect();
        rect.move_to((rect.min.x, y));
        self.set_rect(rect);
    }
    fn set_width(&mut self, width: i32) {
        assert!(width >= 0);
        let mut rect = self.get_rect();
        rect.max.x = rect.min.x + width;
        self.set_rect(rect);
    }
    fn set_height(&mut self, height: i32) {
        assert!(height >= 0);
        let mut rect = self.get_rect();
        rect.max.y = rect.min.y + height;
        self.set_rect(rect);
    }
    fn set_id(&mut self, id: &str);
    fn get_id(&self) -> String;
    fn as_container(&self) -> Option<&dyn Container>;
    fn as_container_mut(&mut self) -> Option<&mut dyn Container>;

    // Events and listeners
    fn onclick(&mut self, func: Box<dyn FnMut(&mut UI, &dyn View) -> bool>);
    fn click(&self, ui: &mut UI) -> bool;

    #[allow(unused_variables)]
    fn on_mouse_move(&self, ui: &mut UI, position: Vector2<i32>) -> bool { false }
    #[allow(unused_variables)]
    fn on_mouse_button_down(&self, ui: &mut UI, position: Vector2<i32>, button: MouseButton) -> bool { false }
    #[allow(unused_variables)]
    fn on_mouse_button_up(&self, ui: &mut UI, position: Vector2<i32>, button: MouseButton) -> bool { false }
    #[allow(unused_variables)]
    fn on_key_down(&self, ui: &mut UI, virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode, state: ModifiersState) -> bool { false }
    #[allow(unused_variables)]
    fn on_key_up(&self, ui: &mut UI, virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode, state: ModifiersState) -> bool { false }
    #[allow(unused_variables)]
    fn on_key_char(&self, ui: &mut UI, unicode_codepoint: char, state: ModifiersState) -> bool { false }
    #[allow(unused_variables)]
    fn on_key_mod_changed(&self, ui: &mut UI, state: ModifiersState) -> bool { false }
}

impl_downcast!(View);

pub trait Container: View {
    fn add_view(&mut self, view: Element);
    fn get_view(&self, id: &str) -> Option<Element>;
    fn get_view_count(&self) -> usize;
}
