use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::{frame::Frame, player::MAX_AMMO, NUM_COLS};
use std::io::{Stdout, Write};

fn clear(stdout: &mut Stdout, clear_type: ClearType) {
    stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
    stdout.queue(Clear(clear_type)).unwrap();
    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
}

pub fn render(
    stdout: &mut Stdout,
    last_frame: &Frame,
    current_frame: &Frame,
    count_shot: u32,
    force: bool,
) {
    if force {
        clear(stdout, ClearType::All);
    }

    // Render frame
    for (y, row) in current_frame.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            if *s != last_frame[y][x] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s);
            }
        }
    }

    // Render UI
    stdout.queue(MoveTo(NUM_COLS as u16 + 5, 4)).unwrap();
    clear(stdout, ClearType::UntilNewLine);
    print!("  ammo: {} / 100  ", (MAX_AMMO - count_shot));

    // Flush
    stdout.flush().unwrap();
}
