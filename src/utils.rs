use macroquad::prelude::*;

pub fn point_in_rect(x: f32, y: f32, rect: Rect) -> bool {
    x > rect.x && x - rect.x < rect.w && 
        y > rect.y && y - rect.y < rect.h
}

// pub fn mouse_state_in_rect(mouse_state: MouseState, rect: Rect) -> bool {
//     point_in_rect(mouse_state.x(), mouse_state.y(), rect)
// }

pub fn centered_rect(x:f32, y:f32, w:f32, h:f32) -> Rect {
    Rect::new(
        x - (w/2.),
        y - (h/2.),
        w,
        h,
    )
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub fn top_midpoint(rect: Rect) -> (f32, f32) {
    let x = rect.x + rect.w / 2.;
    let y = rect.y;
    (x, y)
}
pub fn bottom_midpoint(rect: Rect) -> (f32, f32) {
    let x = rect.x + rect.w / 2.;
    let y = rect.y + rect.h;
    (x, y)
}
pub fn left_midpoint(rect: Rect) -> (f32, f32) {
    let x = rect.x;
    let y = rect.y + rect.h / 2.;
    (x, y)
}
pub fn right_midpoint(rect: Rect) -> (f32, f32) {
    let x = rect.x + rect.w;
    let y = rect.y + rect.h / 2.;
    (x, y)
}

pub fn direction_midpoint(rect: Rect, dir: u8) -> (f32, f32) {
    match dir {
        0 => top_midpoint(rect),
        1 => right_midpoint(rect),
        2 => bottom_midpoint(rect),
        3 => left_midpoint(rect),
        _ => unreachable!(),
    }
}

pub fn draw_texture_to_rect(texture: Texture2D, rect: Rect) {
    let (x, y) = (rect.x, rect.y);
    let dest_size = Some(Vec2::new(rect.w, rect.h));
    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams { 
            dest_size,
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        }
    );
}

pub fn find_min_f32(v: &[f32]) -> f32 {
    let mut v_iter = v.iter();
    let mut min = v_iter.next().unwrap();

    while let Some(value) = v_iter.next() {
        if value < min {
            min = value;
        }
    }

    *min
}