use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
    }
}

fn main() {
    let _clean_up = CleanUp;

    terminal::enable_raw_mode().expect("Could not turn on raw mode");

    loop {
        if let Event::Key(event) = event::read().expect("Failed to read line") {
            match event {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => break,
                _ => {}
            }

            println!("{:?}\r", event);
        };
    }
}
