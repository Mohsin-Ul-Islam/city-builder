use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture, Transformable, View};
use sfml::system::Vector2f;
use sfml::window::{Event, Key, Style};

struct GameAssets<'a> {
    grass: Sprite<'a>,
}

fn main() {
    let width_in_pixels: i32 = 800;
    let height_in_pixels: i32 = 400;

    let tile_size_in_pixels: i32 = 8;
    let camera_scroll_speed: f32 = 16.0;

    let n_rows = height_in_pixels / tile_size_in_pixels;
    let n_cols = width_in_pixels / tile_size_in_pixels;

    let mut center = Vector2f::new(200.0, 100.0);
    let mut view = View::new(center, center);

    let mut window = RenderWindow::new(
        (width_in_pixels as u32, height_in_pixels as u32),
        "City Builder",
        Style::CLOSE,
        &Default::default(),
    );

    window.set_framerate_limit(30);

    let grass_texture = Texture::from_file("assets/images/grass.png").unwrap();
    let mut game_assets = GameAssets {
        grass: Sprite::with_texture(&grass_texture),
    };

    game_assets.grass.set_origin((4.0, 4.0));

    let world = vec![vec![0; n_cols as usize]; n_rows as usize];

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed {
                    code: Key::RIGHT, ..
                } => {
                    center = Vector2f::new(center.x + camera_scroll_speed, center.y);
                    view.set_center(center);
                }
                Event::KeyPressed {
                    code: Key::LEFT, ..
                } => {
                    center = Vector2f::new(center.x - camera_scroll_speed, center.y);
                    view.set_center(center);
                }
                Event::KeyPressed { code: Key::UP, .. } => {
                    center = Vector2f::new(center.x, center.y - camera_scroll_speed);
                    view.set_center(center);
                }
                Event::KeyPressed {
                    code: Key::DOWN, ..
                } => {
                    center = Vector2f::new(center.x, center.y + camera_scroll_speed);
                    view.set_center(center);
                }

                _ => continue,
            }
        }

        window.clear(Color::WHITE);

        for j in 0..n_rows {
            for i in 0..n_cols {
                let x = (i - j) * tile_size_in_pixels + n_cols * tile_size_in_pixels
                    - width_in_pixels / 2;
                let y = (i + j) * tile_size_in_pixels / 2;

                if world[j as usize][i as usize] == 0 {
                    game_assets.grass.set_position((x as f32, y as f32));
                    window.draw(&game_assets.grass);
                }
            }
        }

        window.set_view(&view);
        window.display();
    }
}
