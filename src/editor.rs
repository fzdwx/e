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

pub struct Editor {
    cursor: Cursor,
    fd: std::io::Stdout,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            cursor: Cursor { x: 0, y: 0 },
            fd: stdout(),
        }
    }
}

impl Editor {
    pub async fn react() -> Result<()> {
        let mut editor = Self::default();

        editor.init_screen().await?;

        editor.events().await?;

        editor.exit_screen().await?;

        Ok(())
    }

    async fn events(&mut self) -> Result<()> {
        let mut reader = EventStream::new();

        loop {
            let event = reader.next().fuse();
            self.refresh_screen().await?;
            match event.await {
                Some(Ok(event)) => {
                    if self.dispatch_event(event).await {
                        break;
                    }
                }
                Some(Err(e)) => println!("Error: {:?}\r", e),
                None => break,
            }
        }

        Ok(())
    }


    async fn dispatch_event(&mut self, event: Event) -> bool {
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

    async fn refresh_screen(&mut self) -> Result<()> {
        execute!(self.fd, Hide)?;
        execute!(self.fd, MoveTo(0, 0))?;

        self.draw_rows().await?;

        execute!(self.fd, MoveTo(0, 0))?;
        execute!(self.fd, Show)?;

        Ok(())
    }

    async fn draw_rows(&mut self) -> Result<()> {
        let size = window_size()?;
        let mut fd = stdout();
        for y in 0..size.rows {
            if y == size.rows / 3 {
                let welcome = format!("e -- version {}", env!("CARGO_PKG_VERSION"));
                let padding = (size.columns as usize - welcome.len()) / 2;
                if padding > 0 {
                    write!(fd, "~")?;
                    for _ in 0..padding - 1 {
                        write!(fd, " ")?;
                    }
                    write!(fd, "{}", welcome)?;
                }
            } else {
                write!(fd, "~")?;
            }

            execute!(fd, Clear(ClearType::UntilNewLine))?;

            if y < size.rows - 1 {
                write!(fd, "\r\n")?;
            }
        }

        Ok(())
    }


    async fn init_screen(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(self.fd, EnterAlternateScreen)?;
        execute!(self.fd, EnableMouseCapture)?;
        Ok(())
    }

    async fn exit_screen(&mut self) -> Result<()> {
        execute!(self.fd, DisableMouseCapture)?;
        execute!(self.fd, Clear(ClearType::All))?;
        disable_raw_mode()?;
        Ok(())
    }
}

struct Cursor {
    x: usize,
    y: usize,
}