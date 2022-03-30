use crate::constants::{ASPECT_RATIO, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::map::Map;
use crate::tilemap::TileMapConfig;
use crate::utils;

use sfml::audio::Music;
use sfml::graphics::*;
use sfml::system::{Clock, Vector2f};
use sfml::window::{Cursor, Event, Key, Style};

pub struct Game {}

impl Game {
    pub fn run() {
        // run background music as the game starts
        let mut background_music = Music::from_file("assets/audio/background.ogg").unwrap();
        background_music.play();

        // load cursor image
        let cursor_image = Image::from_file("assets/images/cursor.png").unwrap();

        // camera movement speed
        let camera_scroll_speed: f32 = 1000.0;

        // the center of playable screen
        let camera_viewport = Vector2f::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);
        let mut camera_position =
            Vector2f::new((SCREEN_WIDTH / 2) as f32, (SCREEN_HEIGHT / 2) as f32);

        // our camera object, initially at center
        let mut camera = View::new(camera_position, camera_viewport);
        let mut window = RenderWindow::new(
            (SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
            "City Builder",
            Style::DEFAULT,
            &Default::default(),
        );

        // limit framerate to 30 frames a second
        window.set_framerate_limit(30);

        // set game cursor
        unsafe {
            let cursor = Cursor::from_pixels(
                cursor_image.pixel_data(),
                cursor_image.size(),
                cursor_image.size(),
            )
            .unwrap();
            window.set_mouse_cursor(&cursor);
        }

        // load all game assets
        let sprite_sheet_texture =
            Texture::from_file("assets/spritesheets/building_tiles.png").unwrap();
        let sprite_config =
            TileMapConfig::from_json_file("assets/spritesheets/building_tiles.json");

        // initialize the game assets struct
        let mut sprite_sheet = Sprite::with_texture(&sprite_sheet_texture);

        // 2D world
        let world = Map::from_json_file("assets/maps/default.json");

        // clock
        let mut clock = Clock::start();

        while window.is_open() {
            let elapsed = clock.restart().as_seconds();

            // handle input for navigation keys
            if Key::D.is_pressed() {
                camera_position.x += camera_scroll_speed * elapsed;
            }
            if Key::A.is_pressed() {
                camera_position.x -= camera_scroll_speed * elapsed;
            }
            if Key::W.is_pressed() {
                camera_position.y -= camera_scroll_speed * elapsed;
            }
            if Key::S.is_pressed() {
                camera_position.y += camera_scroll_speed * elapsed;
            }

            camera.set_center(camera_position);

            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed => window.close(),

                    // handle input for mouse scrollwheel zoom in/out
                    Event::MouseWheelScrolled { delta, .. } => {
                        let mut size = camera.size();
                        if delta < 0.0
                            && (size.y as i32) < SCREEN_HEIGHT * 2
                            && (size.x as i32) < SCREEN_WIDTH * 2
                        {
                            size.y += 100.0 * ASPECT_RATIO;
                            size.x += 100.0 * ASPECT_RATIO;
                        } else if delta >= 0.0
                            && (size.y as i32) > SCREEN_HEIGHT
                            && (size.x as i32) > SCREEN_WIDTH
                        {
                            size.y -= 100.0 * ASPECT_RATIO;
                            size.x -= 100.0 * ASPECT_RATIO;
                        }
                        camera.set_size(size);
                    }
                    _ => continue,
                }
            }

            window.clear(Color::WHITE);

            for (i, row) in world.iter().enumerate() {
                for (j, tile_id) in row.iter().enumerate() {
                    // skip rendering if tile is blank
                    if *tile_id == -1 {
                        continue;
                    }

                    let tile = sprite_config.at(tile_id);

                    sprite_sheet.set_texture_rect(&IntRect {
                        top: tile.top,
                        left: tile.left,
                        width: tile.width,
                        height: tile.height,
                    });

                    sprite_sheet.set_position(utils::isometric_to_screen(
                        i as i32,
                        j as i32,
                        &sprite_sheet,
                    ));
                    window.draw(&sprite_sheet);
                }
            }

            window.set_view(&camera);
            window.display();
        }
    }
}
