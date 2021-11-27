#![windows_subsystem = "windows"]
#[macro_use]
extern crate downcast_rs;
extern crate include_dir;
extern crate quick_xml;
extern crate speedy2d;

use speedy2d::dimen::Vector2;
use speedy2d::Window;
use speedy2d::window::{WindowCreationOptions, WindowPosition, WindowSize};

use gui::*;
use gui::themes::Theme;
use gui::ui::UI;
use gui::win::Win;
use themes::Classic;
use traits::View;
use views::{Button, Edit};

mod gui;
#[cfg(tests)]
mod tests;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const TITLE: &'static str = "VinX";

fn main() {
    let layout = include_str!("../layout.xml");
    let ui = UI::from_xml(layout, WIDTH, HEIGHT, Classic::typeface()).unwrap();

    if let Some(button) = ui.get_view("btn1") {
        button.borrow_mut().onclick(Box::new(button1_click));
    }

    let win = Win::new(ui);
    let window_size = WindowSize::PhysicalPixels(Vector2::new(WIDTH, HEIGHT));
    let options = WindowCreationOptions::new_windowed(window_size, Some(WindowPosition::Center));
    let window: Window<String> = Window::new_with_user_events(TITLE, options).unwrap();
    window.run_loop(win);
}

fn button1_click(ui: &mut UI, view: &dyn View) -> bool {
    // Change something in another view
    if let Some(edit) = ui.get_view("edit1") {
        if let Some(e) = edit.borrow_mut().downcast_mut::<Edit>() {
            e.set_text("Button clicked!");
        }
    }
    // Change something in clicked view
    if let Some(button) = view.as_any().downcast_ref::<Button>() {
        button.set_text("Button OK!");
    }
    true
}