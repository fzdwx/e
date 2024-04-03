use std::io::stdout;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use crossterm::event::{KeyEvent, KeyModifiers};
use futures::{future::FutureExt, StreamExt};

const HELP: &str = r#"EventStream based on futures_util::Stream with tokio
 - Keyboard, mouse and terminal resize events enabled
 - Hit "c" to print current cursor position
 - Use "q" to quit
"#;

async fn print_events() {
    let mut reader = EventStream::new();

    loop {
        let event = reader.next().fuse();
        match event.await {
            Some(Ok(event)) => {
                if dispatch_event(event).await { break; }
            }
            Some(Err(e)) => println!("Error: {:?}\r", e),
            None => break,
        }
    }
}

async fn dispatch_event(event: Event) -> bool {
    match event {
        Event::Key(k) => {
            match k {
                KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::NONE, kind: _, state: _ } => {
                    return true;
                }
                KeyEvent { code: KeyCode::Char('c'), modifiers: _, kind: _, state: _ } => {
                    let (x, y) = crossterm::cursor::position().unwrap();
                    println!("Cursor position: x={}, y={}\r", x, y);
                }
                _ => {
                    println!("Event::{:?}\r", k);
                }
            }
        }
        _ => {
            println!("Event::{:?}\r", event);
        }
    }

    false
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", HELP);

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    print_events().await;

    execute!(stdout, DisableMouseCapture)?;

    disable_raw_mode()?;

    Ok(())
}