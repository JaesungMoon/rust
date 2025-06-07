use std::error::Error;
use std::time::Duration;
use crossterm::{event, terminal, ExecutableCommand};
use crossterm::cursor::Hide;
use crossterm::event::{Event, KeyCode};
use rusty_audio::Audio;

fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "./explode.wav");
    audio.add("lose", "./lose.wav");
    audio.add("move", "./move.wav");
    audio.add("pew", "./pew.wav");
    audio.add("startup", "./startup.wav");
    audio.add("win", "./win.wav");
    audio.add("start", "./start.mp3");
    
    audio.play("start");
    // audio.play("startup"); // to small sound
    
    // Terminal
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    
    // Game Loop
    'game_loop: loop {
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'game_loop;
                    }
                    _ => {}
                }
            }
        }
    }
    
    Ok(())
}
