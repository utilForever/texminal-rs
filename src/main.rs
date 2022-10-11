use crossterm::event::*;
use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, terminal};
use std::io::stdout;
use std::time::Duration;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        Output::clear_screen().expect("Error");
    }
}

struct Reader;

impl Reader {
    fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if poll(Duration::from_millis(500))? {
                if let Event::Key(event) = read()? {
                    return Ok(event);
                }
            }
        }
    }
}

struct Output;

impl Output {
    fn new() -> Self {
        Self
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    fn refresh_screen(&self) -> crossterm::Result<()> {
        Self::clear_screen()
    }
}

struct Editor {
    reader: Reader,
    output: Output,
}

impl Editor {
    fn new() -> Self {
        Self {
            reader: Reader,
            output: Output::new(),
        }
    }

    fn process_key_press(&self) -> crossterm::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => return Ok(false),
            _ => {}
        }

        Ok(true)
    }

    fn run(&self) -> crossterm::Result<bool> {
        self.output.refresh_screen()?;
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
