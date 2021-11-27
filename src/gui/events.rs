#[allow(dead_code)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum UiEvent {
    Click,
    MouseDown { x: u32, y: u32 },
    MouseMove { x: u32, y: u32 },
    MouseUp { x: u32, y: u32 },
}