use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture, Transformable};
use sfml::window::{Event, Style};

fn main() {
    let mut window = RenderWindow::new(
        (800, 400),
        "City Builder",
        Style::CLOSE,
        &Default::default(),
    );

    window.set_framerate_limit(60);

    let grass_texture = Texture::from_file("assets/images/grass.png").unwrap();
    let mut grass_sprite = Sprite::with_texture(&grass_texture);

    grass_sprite.set_scale((2.0, 2.0));

    loop {
        match window.poll_event() {
            Some(event) => {
                if event == Event::Closed {
                    window.close();
                    break;
                }
            }
            None => continue,
        }

        window.clear(Color::WHITE);
        window.draw(&grass_sprite);
        window.display();
    }
}
