use sdl2::rect::Rect;
use sdl2::mouse::MouseState;

pub fn point_in_rect(x: i32, y: i32, rect: Rect) -> bool {
    x > rect.x() && x - rect.x() < rect.width() as i32 && 
        y > rect.y() && y - rect.y() < rect.height() as i32
}

pub fn mouse_state_in_rect(mouse_state: MouseState, rect: Rect) -> bool {
    point_in_rect(mouse_state.x(), mouse_state.y(), rect)
}