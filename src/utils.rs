use crate::constants;
use sfml::graphics::Sprite;

pub fn isometric_to_screen(x: i32, y: i32, sprite: &Sprite) -> (f32, f32) {
    let rect = sprite.texture_rect();

    let x0 = constants::SCREEN_WIDTH / 2 - rect.width / 2;
    let y0 = rect.height;

    let xs = x0 + (x - y) * rect.width / 2;
    let ys = y0 + (x + y) * rect.height / 2;

    (xs as f32, ys as f32)
}
