use std::marker::PhantomData;
use speedy2d::dimen::Vector2;
use speedy2d::Graphics2D;
use speedy2d::window::{KeyScancode, ModifiersState, MouseButton, VirtualKeyCode, WindowHandler, WindowHelper, WindowStartupInfo};

use gui::ui::UI;
use gui::themes::*;

pub struct Win<T> {
    ui: UI,
    width: u32,
    height: u32,
    mouse_pos: Vector2<i32>,
    mod_state: Option<ModifiersState>,
    t: PhantomData<T>
}

impl<T> Win<T> {
    pub fn new(ui: UI) -> Self {
        Win {
            ui,
            width: 0,
            height: 0,
            mouse_pos: Vector2::new(-1, -1),
            mod_state: None,
            t: PhantomData::default()
        }
    }
}

impl<T> WindowHandler<T> for Win<T> {
    fn on_start(&mut self, helper: &mut WindowHelper<T>, info: WindowStartupInfo) {
        println!("on_start");
        self.width = info.viewport_size_pixels().x;
        self.height = info.viewport_size_pixels().y;
        self.ui.layout(self.width, self.height, info.scale_factor());
        helper.request_redraw();
    }

    fn on_resize(&mut self, helper: &mut WindowHelper<T>, size_pixels: Vector2<u32>) {
        println!("on_resize");
        if size_pixels.x > 0 && size_pixels.y > 0 {
            self.width = size_pixels.x;
            self.height = size_pixels.y;
            self.ui.layout(size_pixels.x, size_pixels.y, helper.get_scale_factor());
            helper.request_redraw();
        }
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<T>, graphics: &mut Graphics2D) {
        let scale = helper.get_scale_factor();
        let mut theme = Classic::new(graphics, self.width as i32, self.height as i32, scale);
        self.ui.paint(&mut theme);
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<T>, position: Vector2<f32>) {
        //println!("Position: {} x {}", position.x, position.y);
        let position = Vector2::new(position.x.round() as i32, position.y.round() as i32);
        self.mouse_pos = position;
        if self.ui.on_mouse_move(position) {
            //self.ui.layout(self.ui.get_width(), self.ui.get_height());
            helper.request_redraw();
        }
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<T>, button: MouseButton) {
        if self.ui.on_mouse_button_down(self.mouse_pos, button) {
            //self.ui.layout(self.ui.get_width(), self.ui.get_height());
            helper.request_redraw();
        }
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper<T>, button: MouseButton) {
        if self.ui.on_mouse_button_up(self.mouse_pos, button) {
            //self.ui.layout(self.ui.get_width(), self.ui.get_height());
            helper.request_redraw();
        }
    }

    fn on_key_down(&mut self, helper: &mut WindowHelper<T>, virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode) {
        println!("KeyCode: {:?}, scancode: {:?} down", virtual_key_code, scancode);
    }

    fn on_key_up(&mut self, helper: &mut WindowHelper<T>, virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode) {
        println!("KeyCode: {:?}, scancode: {:?} up", virtual_key_code, scancode);
    }

    fn on_keyboard_char(&mut self,helper: &mut WindowHelper<T>, unicode_codepoint: char) {
        println!("Codepoint {:?}", unicode_codepoint);
        if unicode_codepoint == 27 as char {
            helper.terminate_loop();
        }
    }

    fn on_keyboard_modifiers_changed(&mut self, helper: &mut WindowHelper<T>, state: ModifiersState) {
        println!("Modifiers: {:?}", &state);
    }
}
