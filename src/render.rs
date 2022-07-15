use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::frame::Frame;
use std::io::{Stdout, Write};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, current_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (y, row) in current_frame.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            if *s != last_frame[y][x] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s);
            }
        }
    }

    stdout.flush().unwrap();
}
