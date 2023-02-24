#![windows_subsystem = "windows"]
#[macro_use]
extern crate downcast_rs;
extern crate include_dir;
extern crate quick_xml;
extern crate speedy2d;
extern crate rand;

use speedy2d::dimen::Vector2;
use speedy2d::Window;
use speedy2d::window::{WindowCreationOptions, WindowPosition, WindowSize};

use gui::*;
use gui::events::EventType;
use gui::themes::Theme;
use gui::ui::UI;
use gui::win::{Win, WinEvent};
use themes::Classic;
use traits::View;
use views::{Button, Edit, CheckBox};

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
        button.borrow_mut().on_event(EventType::Click, Box::new(button1_click));
    }

    let window_size = WindowSize::PhysicalPixels(Vector2::new(WIDTH, HEIGHT));
    let options = WindowCreationOptions::new_windowed(window_size, Some(WindowPosition::Center));
    let window: Window<WinEvent> = Window::new_with_user_events(TITLE, options).unwrap();
    let sender = window.create_user_event_sender();
    let win = Win::new(ui, sender);
    window.run_loop(win);
}

fn button1_click(ui: &mut UI, view: &dyn View) -> bool {
    let mut checked = false;
    if let Some(checkbox) = ui.get_view("checkbox1") {
        if let Some(ch) = checkbox.borrow_mut().downcast_mut::<CheckBox>() {
            checked = ch.is_checked();
        }
    }

    // Change something in another view
    if let Some(edit) = ui.get_view("edit1") {
        if let Some(e) = edit.borrow_mut().downcast_mut::<Edit>() {
            e.set_text(&format!("CheckBox checked = {}", checked));
        }
    }
    // Change something in clicked view
    if let Some(button) = view.as_any().downcast_ref::<Button>() {
        button.set_text("Clicked!");
    }
    true
}