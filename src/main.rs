use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};
use std::time::Duration;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
    }
}

struct Reader;

impl Reader {
    fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}

struct Editor {
    reader: Reader,
}

impl Editor {
    fn new() -> Self {
        Self { reader: Reader }
    }

    fn process_key_press(&self) -> crossterm::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => return Ok(false),
            _ => {}
        }

        Ok(true)
    }

    fn run(&self) -> crossterm::Result<bool> {
        self.process_key_press()
    }
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;

    terminal::enable_raw_mode()?;

    let editor = Editor::new();
    while editor.run()? {}

    Ok(())
}
