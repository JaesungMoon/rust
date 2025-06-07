use std::error::Error;
use std::sync::mpsc;
use std::{io, thread};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use crossterm::{event, terminal, ExecutableCommand};
use crossterm::cursor::Hide;
use crossterm::event::{Event, KeyCode};
use rusty_audio::Audio;
use invaders::{frame, render};
use invaders::frame::{new_frame, Frame};

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
    
    // Render loop in a separate thread
    let (render_tx, render_rx): (Sender<Frame>, Receiver<Frame>) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });
    
    // Game Loop
    'game_loop: loop {
        // Per-frame init
        let curr_frame = new_frame();
        
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
        // Draw & render
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }
    
    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    
    Ok(())
}
