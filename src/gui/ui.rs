use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use speedy2d::dimen::Vector2;
use speedy2d::window::MouseButton;

use gui::containers::Frame;
use gui::themes::Theme;
use gui::traits::{Element, View};
use gui::types::Point;
use themes::Typeface;
use types::{Rect, rect};

use views::{Button, Edit, Label};

pub struct UI {
    width: u32,
    height: u32,
    typeface: Typeface,
    root: Option<Element>,
    types: HashMap<String, fn() -> Element>
}

#[allow(dead_code)]
impl UI {
    pub fn new(width: u32, height: u32, typeface: Typeface) -> Self {
        let mut ui = UI { width, height, typeface, root: None, types: HashMap::new() };
        ui.register::<Label>("Label");
        ui.register::<Button>("Button");
        ui.register::<Edit>("Edit");
        ui.register::<Frame>("Frame");
        ui
    }

    pub fn add_view(&mut self, view: Element) {
        match &self.root {
            None => {
                self.root = Some(view);
            }
            Some(root) => {
                let mut root = root.try_borrow_mut().unwrap();
                root.as_container_mut().unwrap().add_view(view);
            }
        }
    }

    pub fn get_view(&self, id: &str) -> Option<Element> {
        println!("Searching {} in UI", &id);
        match &self.root {
            None => None,
            Some(root) => {
                if root.borrow().get_id() == id {
                    return Some(Rc::clone(root));
                }
                println!("Searching inside root...");
                root.borrow().as_container().unwrap().get_view(id)
            }
        }
    }

    pub fn register<T: Default + View + 'static>(&mut self, name: &str) {
        self.types.insert(name.to_owned(), || Rc::new(RefCell::from(T::default())));
    }

    pub fn create(&self, name: &str) -> Element {
        self.types.get(name).expect("No type!")()
    }

    pub fn layout(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        let rect = rect((0, 0), (width as i32, height as i32));
        let root = self.root.clone();
        if let Some(root) = root {
            root.try_borrow_mut().unwrap().layout(&rect, &self.typeface.clone());
        }
    }

    pub fn paint(&self, theme: &mut dyn Theme) {
        theme.clear_screen();
        if let Some(root) = &self.root {
            root.borrow().paint(Point::from((0, 0)), theme);
        }
    }

    pub fn from_xml(xml: &str, width: u32, height: u32, typeface: Typeface) -> Option<Self> {
        let mut ui = UI::new(width, height, typeface);
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut txt = Vec::new();
        let mut buf = Vec::new();
        let mut stack: Vec<Element> = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let element = UI::parse_element(&mut ui, e);
                    stack.push(element);
                },
                Ok(Event::Empty(ref e)) => {
                    let element = UI::parse_element(&mut ui, e);
                    let parent = stack.pop().unwrap();
                    {
                        element.borrow_mut().set_parent(Some(Rc::downgrade(&parent)));
                        let mut ref_mut = parent.borrow_mut();
                        let container = ref_mut.as_container_mut().unwrap();
                        container.add_view(element);
                    }
                    stack.push(parent);
                },
                Ok(Event::End(_)) => {
                    // TODO check that it is the same tag
                    let element = stack.pop().unwrap();
                    match stack.pop() {
                        None => {
                            ui.add_view(element);
                        }
                        Some(parent) => {
                            {
                                let mut ref_mut = parent.borrow_mut();
                                let container = ref_mut.as_container_mut().unwrap();
                                container.add_view(element);
                            }
                            stack.push(parent);
                        }
                    }
                },
                // unescape and decode the text event using the reader encoding
                Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        Some(ui)
    }

    fn parse_element(ui: &mut UI, e: &BytesStart) -> Element {
        let attributes = e
            .attributes()
            .map(|a| a.unwrap())
            .collect::<Vec<_>>();
        //println!("attributes values: {:?}", attributes);
        let view_type = String::from_utf8(e.name().to_vec()).unwrap();
        let view = ui.create(&view_type);
        //println!("Loaded {}", &view_type);
        for attribute in attributes {
            let name = String::from_utf8(attribute.key.to_vec()).unwrap();
            let value = match attribute.value {
                Cow::Borrowed(c) => {
                    String::from_utf8(c.to_vec()).unwrap()
                }
                Cow::Owned(c) => {
                    String::from_utf8(c.to_vec()).unwrap()
                }
            };
            view.borrow_mut().set_any(&name, &value);
            //println!("Attribute: {} = {}", &name, &value);
        }
        view
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn on_mouse_move(&mut self, position: Vector2<i32>) -> bool {
        let root = self.root.clone();
        match root {
            None => false,
            Some(root) => {
                root.borrow().on_mouse_move(self, position)
            }
        }
    }

    pub fn on_mouse_button_down(&mut self, position: Vector2<i32>, button: MouseButton) -> bool {
        let root = self.root.clone();
        match root {
            None => false,
            Some(root) => {
                root.borrow().on_mouse_button_down(self, position, button)
            }
        }
    }

    pub fn on_mouse_button_up(&mut self, position: Vector2<i32>, button: MouseButton) -> bool {
        let root = self.root.clone();
        match root {
            None => false,
            Some(root) => {
                root.borrow().on_mouse_button_up(self, position, button)
            }
        }
    }
}