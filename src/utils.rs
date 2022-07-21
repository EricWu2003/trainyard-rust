use sdl2::rect::Rect;
use sdl2::mouse::MouseState;

pub fn point_in_rect(x: i32, y: i32, rect: Rect) -> bool {
    x > rect.x() && x - rect.x() < rect.width() as i32 && 
        y > rect.y() && y - rect.y() < rect.height() as i32
}

pub fn mouse_state_in_rect(mouse_state: MouseState, rect: Rect) -> bool {
    point_in_rect(mouse_state.x(), mouse_state.y(), rect)
}

pub fn centered_rect(x:i32, y:i32, w:u32, h:u32) -> Rect {
    Rect::new(
        x - (w/2) as i32,
        y - (h/2) as i32,
        w,
        h,
    )
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub fn top_midpoint(rect: Rect) -> (i32, i32) {
    let x = rect.x() + rect.width() as i32 /2;
    let y = rect.y();
    (x, y)
}
pub fn bottom_midpoint(rect: Rect) -> (i32, i32) {
    let x = rect.x() + rect.width() as i32 /2;
    let y = rect.y() + rect.height() as i32;
    (x, y)
}
pub fn left_midpoint(rect: Rect) -> (i32, i32) {
    let x = rect.x();
    let y = rect.y() + rect.height() as i32/2;
    (x, y)
}
pub fn right_midpoint(rect: Rect) -> (i32, i32) {
    let x = rect.x() + rect.width() as i32;
    let y = rect.y() + rect.height() as i32/2;
    (x, y)
}

pub fn direction_midpoint(rect: Rect, dir: u8) -> (i32, i32) {
    match dir {
        0 => top_midpoint(rect),
        1 => right_midpoint(rect),
        2 => bottom_midpoint(rect),
        3 => left_midpoint(rect),
        _ => unreachable!(),
    }
}