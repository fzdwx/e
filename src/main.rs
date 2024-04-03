use std::io::stdout;
use std::io::Write;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{KeyEvent, KeyModifiers};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, window_size};
use futures::{future::FutureExt, StreamExt};

#[tokio::main]
async fn main() -> Result<()> {
    init_screen().await?;

    events().await?;

    exit_screen().await?;

    Ok(())
}

async fn events() -> Result<()> {
    let mut reader = EventStream::new();

    loop {
        let event = reader.next().fuse();
        refresh_screen().await?;
        match event.await {
            Some(Ok(event)) => {
                if dispatch_event(event).await {
                    break;
                }
            }
            Some(Err(e)) => println!("Error: {:?}\r", e),
            None => break,
        }
    }

    Ok(())
}

async fn dispatch_event(event: Event) -> bool {
    match event {
        Event::Key(k) => match k {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                kind: _,
                state: _,
            } => {
                return true;
            }
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: _,
                kind: _,
                state: _,
            } => {
                let (x, y) = crossterm::cursor::position().unwrap();
                println!("Cursor position: x={}, y={}\r", x, y);
            }
            _ => {
                println!("Event::{:?}\r", k);
            }
        },
        _ => {
            println!("Event::{:?}\r", event);
        }
    }

    false
}

async fn init_screen() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), EnableMouseCapture)?;
    Ok(())
}

async fn exit_screen() -> Result<()> {
    let mut fd = stdout();

    execute!(fd, DisableMouseCapture)?;
    execute!(fd, Clear(ClearType::All))?;
    disable_raw_mode()?;
    Ok(())
}

async fn refresh_screen() -> Result<()> {
    let mut fd = stdout();

    execute!(fd, Hide)?;
    execute!(fd, MoveTo(0, 0))?;

    draw_rows().await?;

    execute!(fd, MoveTo(0, 0))?;
    execute!(fd, Show)?;

    Ok(())
}

async fn draw_rows() -> Result<()> {
    let size = window_size()?;
    for y in 0..size.rows {
        write!(stdout(), "~")?;
        execute!(stdout(),Clear(ClearType::UntilNewLine))?;

        if y < size.rows - 1 {
            write!(stdout(), "\r\n")?;
        }
    }

    Ok(())
}
