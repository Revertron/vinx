#![windows_subsystem = "windows"]

extern crate speedy2d;
extern crate vinx;

use speedy2d::dimen::Vector2;
use speedy2d::Window;
use speedy2d::window::{WindowCreationOptions, WindowPosition, WindowSize};

use vinx::gui::events::EventType;
use vinx::gui::themes::Theme;
use vinx::gui::ui::UI;
use vinx::gui::views::List;
use vinx::gui::win::{Win, WinEvent};
use vinx::gui::themes::Classic;
use vinx::gui::traits::View;
use vinx::gui::views::{Button, Edit, CheckBox};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const TITLE: &'static str = "VinX";

fn main() {
    let layout = include_str!("layout.xml");
    let mut ui = UI::from_xml(layout, WIDTH, HEIGHT, Classic::typeface()).unwrap();

    if let Some(button) = ui.get_view("btn1") {
        button.borrow_mut().on_event(EventType::Click, Box::new(button1_click));
    }

    if let Some(button) = ui.get_view("btn2") {
        button.borrow_mut().on_event(EventType::Click, Box::new(button2_click));
    }

    ui.on_start(Box::new(on_start));

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

fn button2_click(ui: &mut UI, _view: &dyn View) -> bool {
    let mut buf = Vec::new();
    for i in 1..=20 {
        buf.push(format!("New item {}", i));
    }
    // Set items for list
    if let Some(list) = ui.get_view("list1") {
        if let Some(list) = list.borrow_mut().downcast_mut::<List>() {
            list.set_items(buf);
        }
    }
    true
}

fn on_start(ui: &mut UI) {
    let mut buf = Vec::new();
    for i in 1..=20 {
        buf.push(format!("Start item {}", i));
    }
    // Set items for list
    if let Some(list) = ui.get_view("list1") {
        if let Some(list) = list.borrow_mut().downcast_mut::<List>() {
            list.set_items(buf);
        }
    }
}