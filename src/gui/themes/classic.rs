use std::cmp::{max, min};
use std::collections::VecDeque;
use std::rc::Rc;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::FormattedTextBlock;
use speedy2d::Graphics2D;
use speedy2d::shape::Rectangle;
use gui::styles::selector::{DrawState, MainSelector};
use gui::themes::{Theme, Typeface, ViewState};
use gui::themes::utils::draw_dashed_rectangle;
use gui::types::Rect;
use gui::types;
use gui::types::rect;

#[allow(unused)]
pub struct Classic<'h> {
    graphics: &'h mut Graphics2D,
    width: i32,
    height: i32,
    scale: f64,
    current_clip: Rect<i32>,
    clip_stack: VecDeque<Rect<i32>>
}

#[allow(dead_code)]
impl<'h> Classic<'h> {
    const BACKGROUND: u32 = 0xffd4d0c8;
    const BACKGROUND_LIGHT: u32 = 0xffe4e0d8;
    const BACKGROUND_WHITE: u32 = 0xffffffff;
    const LIGHT: u32 = 0xff808080;
    const DARK: u32 = 0xff404040;
    const BLACK: u32 = 0xff000000;

    pub fn new(graphics: &'h mut Graphics2D, width: i32, height: i32, scale: f64) -> Self {
        let current_clip = rect((0, 0), (width, height));
        Classic { graphics, width, height, scale, current_clip, clip_stack: VecDeque::new() }
    }
}

impl<'h> Theme for Classic<'h> {
    fn clear_screen(&mut self) {
        self.graphics.set_clip(None);
        self.graphics.clear_screen(Color::from_hex_rgb(Classic::BACKGROUND));
        self.set_clip(self.current_clip);
    }

    fn typeface() -> Typeface {
        Typeface::default()
    }

    fn get_back_color(&self, state: ViewState, selector: &MainSelector) -> u32 {
        if let Some(s) = selector.get_state(&state) {
            match s {
                DrawState::Transparent => return 0x00000000,
                DrawState::Color(c) => return *c,
                _ => {}
            }
        }
        Classic::BACKGROUND
    }

    fn get_text_color(&self, state: ViewState, selector: &MainSelector) -> u32 {
        if let Some(s) = selector.get_state(&state) {
            match s {
                DrawState::Transparent => return 0x00000000,
                DrawState::Color(c) => return *c,
                _ => {}
            }
        }
        if !state.enabled {
            return 0xff202020;
        }
        0xff000000
    }

    fn set_clip(&mut self, rect: Rect<i32>) {
        self.current_clip = rect;
        let rect = Rectangle::from_tuples((rect.min.x, rect.min.y), (rect.max.x, rect.max.y));
        self.graphics.set_clip(Some(rect));
    }

    fn clip_rect(&mut self, rect: Rect<i32>) -> Rect<i32> {
        let min_x = max(rect.min.x, self.current_clip.min.x);
        let max_x = min(rect.max.x, self.current_clip.max.x);
        let min_y = max(rect.min.y, self.current_clip.min.y);
        let max_y = min(rect.max.y, self.current_clip.max.y);
        let rect = types::rect((min_x, min_y), (max_x, max_y));
        self.set_clip(rect);
        rect
    }

    fn push_clip(&mut self) {
        self.clip_stack.push_back(self.current_clip);
    }

    fn pop_clip(&mut self) {
        if let Some(clip) = self.clip_stack.pop_back() {
            self.set_clip(clip);
        }
    }

    #[allow(unused)]
    fn draw_button_back(&mut self, rect: Rect<i32>, state: ViewState) {
        let top_left = Vector2::new(rect.min.x as f32, rect.min.y as f32);
        let bottom_right = Vector2::new(rect.max.x as f32, rect.max.y as f32);
        let color = if state.hovered || state.pressed {
            Color::from_hex_rgb(Classic::BACKGROUND_LIGHT)
        } else {
            Color::from_hex_rgb(Classic::BACKGROUND)
        };
        self.graphics.draw_rectangle(Rectangle::new(top_left, bottom_right), color);
    }

    #[allow(unused)]
    fn draw_button_body(&mut self, rect: Rect<i32>, state: ViewState) {
        let border: f32 = self.scale as f32;
        let border_half: f32 = (self.scale / 2f64) as f32;
        let top_left = Vector2::new(rect.min.x as f32, rect.min.y as f32);
        let bottom_right = Vector2::new(rect.max.x as f32, rect.max.y as f32);
        match state.pressed && state.hovered {
            true => {
                let border2: f32 = (self.scale * 2f64) as f32;
                let color = Color::from_hex_rgb(Classic::LIGHT);
                self.graphics.draw_line((top_left.x, top_left.y + border_half), (bottom_right.x - border, top_left.y + border_half), border, color);
                self.graphics.draw_line((top_left.x + border_half, top_left.y), (top_left.x + border_half, bottom_right.y - border), border, color);
                let color = Color::from_hex_rgb(Classic::DARK);
                self.graphics.draw_line((top_left.x + border, top_left.y + border + border_half), (bottom_right.x - border, top_left.y + border + border_half), border, color);
                self.graphics.draw_line((top_left.x + border + border_half, top_left.y + border), (top_left.x + border + border_half, bottom_right.y - border), border, color);

                let color = Color::from_hex_rgb(0xffffff);
                self.graphics.draw_line((top_left.x + border, bottom_right.y - border - border_half), (bottom_right.x - border, bottom_right.y - border - border_half), border, color);
                self.graphics.draw_line((bottom_right.x - border - border_half, top_left.y + border), (bottom_right.x - border - border_half, bottom_right.y - border), border, color);
            }
            false => {
                let color = Color::from_hex_rgb(0xffffff);
                self.graphics.draw_line((top_left.x, top_left.y + border_half), (bottom_right.x - border_half, top_left.y + border_half), border, color);
                self.graphics.draw_line((top_left.x + border_half, top_left.y + border_half), (top_left.x + border_half, bottom_right.y - border_half), border, color);
                let color = Color::from_hex_rgb(Classic::DARK);
                self.graphics.draw_line((top_left.x - border_half, bottom_right.y - border_half), (bottom_right.x, bottom_right.y - border_half), border, color);
                self.graphics.draw_line((bottom_right.x - border_half, top_left.y - border_half), (bottom_right.x - border_half, bottom_right.y + 0.5), border, color);
                let color = Color::from_hex_rgb(Classic::LIGHT);
                self.graphics.draw_line((top_left.x + border, bottom_right.y - border - border_half), (bottom_right.x - border, bottom_right.y - border - border_half), border, color);
                self.graphics.draw_line((bottom_right.x - border - border_half, top_left.y + border), (bottom_right.x - border - border_half, bottom_right.y - border), border, color);
            }
        }
        if state.focused {
            let color = Color::from_hex_rgb(0x000000);
            let padding = border * 4f32;
            draw_dashed_rectangle(self.graphics, top_left.x + padding - 1.0, top_left.y + padding - 1.0, bottom_right.x - padding, bottom_right.y - padding, 2.5f32, border, color);
            //self.graphics.draw_line((top_left.x + border * 4f32, top_left.y + border * 4f32), (bottom_right.x - border * 4f32, top_left.y + border * 4f32), border, color);
            //self.graphics.draw_line((top_left.x + border * 4f32, bottom_right.y - border * 4f32), (bottom_right.x - border * 4f32, bottom_right.y - border * 4f32), border, color);
        }
    }

    #[allow(unused)]
    fn draw_button_text(&mut self, rect: Rect<i32>, state: ViewState, size: usize, text: &str) {
        todo!()
    }

    #[allow(unused)]
    fn draw_edit_back(&mut self, rect: Rect<i32>, state: ViewState) {
        let top_left = Vector2::new(rect.min.x as f32, rect.min.y as f32);
        let bottom_right = Vector2::new(rect.max.x as f32, rect.max.y as f32);
        let color = Color::from_hex_rgb(0xffffff);
        self.graphics.draw_rectangle(Rectangle::new(top_left, bottom_right), color);
    }

    #[allow(unused)]
    fn draw_edit_body(&mut self, rect: Rect<i32>, state: ViewState) {
        let border: f32 = self.scale as f32;
        let border2: f32 = (self.scale * 2f64) as f32;
        let border_half: f32 = (self.scale / 2f64) as f32;
        let top_left = Vector2::new(rect.min.x as f32, rect.min.y as f32);
        let bottom_right = Vector2::new(rect.max.x as f32, rect.max.y as f32);
        let color = Color::from_hex_rgb(Classic::LIGHT);
        self.graphics.draw_line((top_left.x, top_left.y + border_half), (bottom_right.x - border, top_left.y + border_half), border, color);
        self.graphics.draw_line((top_left.x + border_half, top_left.y), (top_left.x + border_half, bottom_right.y - border), border, color);
        let color = Color::from_hex_rgb(Classic::DARK);
        self.graphics.draw_line((top_left.x + border, top_left.y + border + border_half), (bottom_right.x - border, top_left.y + border + border_half), border, color);
        self.graphics.draw_line((top_left.x + border + border_half, top_left.y + border), (top_left.x + border + border_half, bottom_right.y - border), border, color);

        let color = Color::from_hex_rgb(Classic::BACKGROUND);
        self.graphics.draw_line((top_left.x + border, bottom_right.y - border - border_half), (bottom_right.x - border, bottom_right.y - border - border_half), border, color);
        self.graphics.draw_line((bottom_right.x - border - border_half, top_left.y + border), (bottom_right.x - border - border_half, bottom_right.y - border), border, color);

        let color = Color::from_hex_rgb(0xffffff);
        self.graphics.draw_line((top_left.x + border, bottom_right.y - border_half), (bottom_right.x - border, bottom_right.y - border_half), border, color);
    }

    fn draw_edit_caret(&mut self, rect: Rect<i32>, state: ViewState) {
        if !state.focused {
            return;
        }
        let top_left = Vector2::new(rect.min.x as f32, rect.min.y as f32);
        let bottom_right = Vector2::new(rect.max.x as f32, rect.max.y as f32);
        let color = Color::from_hex_rgb(Classic::BLACK);
        self.graphics.draw_rectangle(Rectangle::new(top_left, bottom_right), color);
    }

    fn draw_checkbox_back(&mut self, rect: Rect<i32>, state: ViewState) {
        self.draw_edit_back(rect, state);
    }

    fn draw_checkbox_body(&mut self, rect: Rect<i32>, state: ViewState) {
        self.draw_edit_body(rect, state);
        if state.checked {
            let top_left = Vector2::new(rect.min.x as f32 + self.scale as f32 * 3.0, rect.min.y as f32 + self.scale as f32 * 3.0);
            let bottom_right = Vector2::new(rect.max.x as f32 - self.scale as f32 * 3.0, rect.max.y as f32 - self.scale as f32 * 3.0);
            let width = bottom_right.x - top_left.x;
            let height = bottom_right.y - top_left.y;
            let color = Color::from_hex_rgb(Classic::BLACK);
            self.graphics.draw_line((top_left.x, top_left.y + height / 2f32), (top_left.x + width / 3f32, bottom_right.y - height / 8f32), self.scale as f32, color);
            self.graphics.draw_line((top_left.x + width / 3f32, bottom_right.y - height / 8f32), (bottom_right.x, top_left.y + height / 8f32), self.scale as f32, color);
        }
    }

    fn draw_list_back(&mut self, rect: Rect<i32>, state: ViewState) {
        self.draw_edit_back(rect, state);
    }

    fn draw_list_body(&mut self, rect: Rect<i32>, state: ViewState) {
        self.draw_edit_body(rect, state);
    }

    #[allow(unused)]
    fn draw_panel_back(&mut self, rect: Rect<i32>, state: ViewState) {
        let top_left = Vector2::new(rect.min.x as f32, rect.min.y as f32);
        let bottom_right = Vector2::new(rect.max.x as f32, rect.max.y as f32);
        let color = Color::from_hex_rgb(Classic::BACKGROUND);
        self.graphics.draw_rectangle(Rectangle::new(top_left, bottom_right), color);
    }

    #[allow(unused)]
    fn draw_panel_body(&mut self, rect: Rect<i32>, state: ViewState) {
        let top_left = Vector2::new(rect.min.x as f32, rect.min.y as f32);
        let bottom_right = Vector2::new(rect.max.x as f32, rect.max.y as f32);
        let border: f32 = 1f32;
        let color = Color::from_hex_rgb(0xff808080);
        let half = 0.5f32;
        //draw_rounded_rectangle(self.graphics, rect.min.x as f32, rect.min.y as f32, rect.max.x as f32, rect.max.y as f32, 16f32, 2f32, color);
        self.graphics.draw_line((top_left.x, top_left.y + border - half), (bottom_right.x, top_left.y + border - half), border, color);
        self.graphics.draw_line((top_left.x, bottom_right.y - half), (bottom_right.x, bottom_right.y - half), border, color);
        self.graphics.draw_line((top_left.x + half, top_left.y + border), (top_left.x + half, bottom_right.y + border), border, color);
        self.graphics.draw_line((bottom_right.x - half, top_left.y + border + half), (bottom_right.x - half, bottom_right.y + border - half), border, color);
    }

    fn draw_text(&mut self, x: f32, y: f32, color: u32, text: &Rc<FormattedTextBlock>) {
        let color = Color::from_hex_rgb(color);
        self.graphics.draw_text((x, y), color, text);
    }

    fn draw_rect(&mut self, rect: Rect<i32>, color: u32) {
        let top_left = Vector2::new(rect.min.x as f32, rect.min.y as f32);
        let bottom_right = Vector2::new(rect.max.x as f32, rect.max.y as f32);
        let color = Color::from_hex_argb(color);
        self.graphics.draw_rectangle(Rectangle::new(top_left, bottom_right), color);
    }
}