use sfml::window::{Event, Style};
use sfml::graphics::{RenderWindow, RenderTarget, Color};

fn main() {

    let mut window = RenderWindow::new(
        (800, 400),
        "City Builder",
        Style::CLOSE,
        &Default::default(),
    );

    window.set_framerate_limit(60);

    loop {
        match window.poll_event() {
            Some(event) => match event {
                Event::Closed => return window.close(),
                _ => (), 
            },
            None => continue,
        }
        window.clear(Color::WHITE);
        window.display();
    }
}
