use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::window::{Event, Style};

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
            Some(event) => {
                if event == Event::Closed {
                    window.close();
                    break;
                }
            }
            None => continue,
        }
        window.clear(Color::WHITE);
        window.display();
    }
}
