use include_dir::{Dir, include_dir};
use speedy2d::font::Font;
use std::cell::RefCell;
use std::collections::HashMap;

const ASSETS: Dir = include_dir!("assets");

thread_local! {
    static FONTS: RefCell<HashMap<(String, String), Font>> = RefCell::new(HashMap::new());
}

pub fn get_font(name: &str, style: &str) -> Option<Font> {
    let key = (name.replace(" ", ""), style.replace(" ", ""));

    FONTS.with(|fonts| {
        if !fonts.borrow().contains_key(&key) {
            let separator = std::path::MAIN_SEPARATOR;
            if let Some(file) = ASSETS.get_file(format!("fonts{}{}-{}.ttf", separator, &key.0, &key.1)) {
                match Font::new(file.contents()) {
                    Ok(font) => {
                        fonts.borrow_mut().insert(key.clone(), font);
                    }
                    Err(_) => {
                        println!("Error parsing font file from assets!");
                    }
                }
            }
        }
        fonts.borrow().get(&key).cloned()
    })
}