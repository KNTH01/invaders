use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use invaders::{
    frame::{self, new_frame, Drawable},
    invaders::Invaders,
    player::{Player, MAX_AMMO},
    render,
};

use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Watch out for the Invaders!");

    let mut audio = rusty_audio::Audio::new();
    audio.add("startup", "assets/sounds/startup.wav");
    audio.add("lose", "assets/sounds/lose.wav");
    audio.add("explode", "assets/sounds/explode.wav");
    audio.add("move", "assets/sounds/move.wav");
    audio.add("pew", "assets/sounds/pew.wav");
    audio.add("meow", "assets/sounds/meow.wav");
    audio.add("win", "assets/sounds/win.wav");

    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();

    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();

        render::render(&mut stdout, &last_frame, &last_frame, 0, true);

        while let Ok((current_frame, count_shot)) = render_rx.recv() {
            render::render(&mut stdout, &last_frame, &current_frame, count_shot, false);
            last_frame = current_frame;
        }
    });

    // Game Loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    'gameloop: loop {
        // Per frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut current_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        } else if player.count_shot >= MAX_AMMO {
                            audio.play("meow");
                        }
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move")
        }
        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        // Draw & render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut current_frame);
        }

        let _ = render_tx.send((current_frame, player.count_shot));
        thread::sleep(Duration::from_millis(1));

        // Win or Lose?
        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }
        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();

    audio.wait();

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;

    terminal::disable_raw_mode()?;

    println!("You have shot {} times!", player.count_shot);

    Ok(())
}
