use std::rc::Rc;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::FormattedTextBlock;
use speedy2d::Graphics2D;
use speedy2d::shape::Rectangle;
use gui::themes::{Theme, Typeface, ViewState};
use gui::types::Rect;

pub struct Classic<'h> {
    graphics: &'h mut Graphics2D,
    width: i32,
    height: i32,
    scale: f64,
}

#[allow(dead_code)]
impl<'h> Classic<'h> {
    const BACKGROUND: u32 = 0xffd4d0c8;
    const BACKGROUND_LIGHT: u32 = 0xffe4e0d8;
    const LIGHT: u32 = 0xff808080;
    const DARK: u32 = 0xff404040;
    const BLACK: u32 = 0xff000000;

    pub fn new(graphics: &'h mut Graphics2D, width: i32, height: i32, scale: f64) -> Self {
        Classic { graphics, width, height, scale }
    }
}

impl<'h> Theme for Classic<'h> {
    fn clear_screen(&mut self) {
        self.graphics.clear_screen(Color::from_hex_rgb(Classic::BACKGROUND));
    }

    fn typeface() -> Typeface {
        Typeface::default()
    }

    fn set_clip(&mut self, rect: Rect<i32>) {
        let rect = Rectangle::from_tuples((rect.min.x, self.height - rect.max.y), (rect.max.x, self.height - rect.min.y));
        self.graphics.set_clip(Some(rect));
    }

    #[allow(unused)]
    fn draw_button_back(&mut self, rect: Rect<i32>, state: ViewState) {
        let top_left = Vector2::new(rect.min.x as f32, rect.min.y as f32);
        let bottom_right = Vector2::new(rect.max.x as f32, rect.max.y as f32);
        let color = match state {
            ViewState::Hovered => Color::from_hex_rgb(Classic::BACKGROUND_LIGHT),
            ViewState::Pressed => Color::from_hex_rgb(Classic::BACKGROUND_LIGHT),
            _ => Color::from_hex_rgb(Classic::BACKGROUND)
        };
        self.graphics.draw_rectangle(Rectangle::new(top_left, bottom_right), color);
    }

    #[allow(unused)]
    fn draw_button_body(&mut self, rect: Rect<i32>, state: ViewState) {
        let border: f32 = self.scale as f32;
        let border_half: f32 = (self.scale / 2f64) as f32;
        let top_left = Vector2::new(rect.min.x as f32, rect.min.y as f32);
        let bottom_right = Vector2::new(rect.max.x as f32, rect.max.y as f32);
        match state {
            ViewState::Pressed => {
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
            _ => {
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
    }

    #[allow(unused)]
    fn draw_panel_back(&mut self, rect: Rect<i32>, state: ViewState) {
        todo!()
    }

    #[allow(unused)]
    fn draw_panel_body(&mut self, rect: Rect<i32>, state: ViewState) {
        todo!()
    }

    fn draw_text(&mut self, x: f32, y: f32, text: &Rc<FormattedTextBlock>) {
        self.graphics.draw_text((x, y), Color::DARK_GRAY, text);
    }
}