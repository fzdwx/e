use std::io::stdout;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{KeyEvent, KeyModifiers};
use crossterm::terminal::{window_size, Clear, ClearType, EnterAlternateScreen};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use futures::{future::FutureExt, StreamExt};

use crate::ropex::write_slices;
use crate::{cursor, doc, Args};

pub struct Editor {
    cursor: cursor::Cursor,
    fd: std::io::Stdout,
    document: doc::Document,
    size: TermSize,
}

pub struct TermSize {
    pub rows: usize,
    pub columns: usize,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            cursor: cursor::Cursor::default(),
            fd: stdout(),
            document: "Hello world".into(),
            size: TermSize {
                rows: 0,
                columns: 0,
            },
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
            Event::Resize(c, r) => {
                self.size.columns = c as usize;
                self.size.rows = r as usize;
            }
            _ => {}
        }

        if self.cursor.react(event, self.document.get_lines()).await? {
            return Ok(true);
        }

        Ok(false)
    }

    async fn refresh_screen(&mut self) -> Result<()> {
        self.cursor.scroll(&self.size);
        execute!(self.fd, Hide)?;
        execute!(self.fd, MoveTo(0, 0))?;

        self.draw_rows().await?;

        execute!(self.fd, MoveTo(
            (self.cursor.x - self.cursor.col_offset) as u16,
            (self.cursor.y - self.cursor.row_offset) as u16))?;
        execute!(self.fd, Show)?;

        Ok(())
    }

    async fn draw_rows(&mut self) -> Result<()> {
        let mut fd = stdout();
        for y in 0..self.size.rows {
            let current_row = y + self.cursor.row_offset;
            if current_row >= self.document.get_lines() {
                if self.document.get_lines() == 0 && y == (self.size.rows / 3) as usize {
                    let welcome = format!("e -- version {}", env!("CARGO_PKG_VERSION"));
                    let padding = (self.size.columns - welcome.len()) / 2;
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
                    write_slices(&mut fd, line, self.cursor.col_offset)?;
                }
            }
            execute!(fd, Clear(ClearType::UntilNewLine))?;

            if y < (self.size.rows - 1) as usize {
                write!(fd, "\r\n")?;
            }
        }

        Ok(())
    }

    async fn init_screen(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(self.fd, EnterAlternateScreen)?;
        execute!(self.fd, EnableMouseCapture)?;

        let size = window_size()?;
        self.size = TermSize {
            rows: size.rows as usize,
            columns: size.columns as usize,
        };

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
