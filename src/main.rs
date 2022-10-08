use crossterm::terminal;
use std::io::{self, Read};

fn main() {
    terminal::enable_raw_mode().expect("Could not turn on raw mode");

    let mut buf = [0; 1];

    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {}
}
