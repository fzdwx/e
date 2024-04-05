use std::io::stdout;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{KeyEvent, KeyModifiers};
use crossterm::terminal::{window_size, Clear, ClearType, EnterAlternateScreen, WindowSize};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use futures::{future::FutureExt, StreamExt};
use termion::input::TermRead;

use crate::ropex::write_slices;
use crate::{cursor, doc, Args};

pub struct Editor {
    cursor: cursor::Cursor,
    fd: std::io::Stdout,
    document: doc::Document,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            cursor: cursor::Cursor::default(),
            fd: stdout(),
            document: "Hello world".into(),
        }
    }
}

impl Editor {
    pub async fn react(args: &Args) -> Result<()> {
        let mut editor = Self::default();
        if args.file.is_some() {
            editor.open(args.file.as_ref().unwrap().as_path()).await?;
        }

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
                    if self.dispatch_event(event).await? {
                        break;
                    }
                }
                Some(Err(e)) => println!("Error: {:?}\r", e),
                None => break,
            }
        }

        Ok(())
    }

    async fn dispatch_event(&mut self, event: Event) -> Result<bool> {
        match event {
            Event::Key(k) => match k {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                } => {
                    return Ok(true);
                }

                _ => {}
            },
            _ => {}
        }

        if self.cursor.react(event, self.document.get_lines()).await? {
            return Ok(true);
        }

        Ok(false)
    }

    async fn refresh_screen(&mut self) -> Result<()> {
        let size = window_size()?;
        self.cursor.scroll(&size);
        execute!(self.fd, Hide)?;
        execute!(self.fd, MoveTo(0, 0))?;

        self.draw_rows(&size).await?;

        execute!(self.fd, MoveTo(self.cursor.x as u16, self.cursor.y as u16))?;
        execute!(self.fd, Show)?;

        Ok(())
    }

    async fn draw_rows(&mut self, size: &WindowSize) -> Result<()> {
        let mut fd = stdout();
        for y in 0..size.rows as usize {
            let current_row = y + self.cursor.row_offset;
            if current_row >= self.document.get_lines() {
                if self.document.get_lines() == 0 && y == (size.rows / 3) as usize {
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
            } else {
                if let Some(line) = self.document.text.get_line(current_row) {
                    write_slices(&mut fd, line)?;
                }
            }
            execute!(fd, Clear(ClearType::UntilNewLine))?;

            if y < (size.rows - 1) as usize {
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

    async fn open(&mut self, p: &Path) -> Result<()> {
        self.document = doc::Document::open(p)?;
        Ok(())
    }
}
