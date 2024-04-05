use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{window_size, WindowSize};

pub struct Cursor {
    /// column
    pub x: u16,
    /// row
    pub y: u16,
}

impl Cursor {
    pub async fn react(&mut self, event: Event) -> Result<bool> {
        let size = window_size()?;
        match event {
            Event::Key(k) => match k {
                KeyEvent { .. } => {
                    if self.should_move_up(k) {
                        self.move_up(&size);
                    } else if self.should_move_down(k) {
                        self.move_down(&size);
                    } else if self.should_move_left(k) {
                        self.move_left(&size);
                    } else if self.should_move_right(k) {
                        self.move_right(&size);
                    } else if self.should_page_up(k) {
                        self.page_up(&size);
                    } else if self.should_page_down(k) {
                        self.page_down(&size);
                    } else if self.should_home(k) {
                        self.home(&size);
                    } else if self.should_end(k) {
                        self.end(&size);
                    }
                }
            },
            _ => {}
        }

        Ok(false)
    }

    fn move_up(&mut self, _: &WindowSize) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    fn move_down(&mut self, size: &WindowSize) {
        if self.y != size.rows - 1 {
            self.y += 1;
        }
    }

    fn move_left(&mut self, _: &WindowSize) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    fn move_right(&mut self, size: &WindowSize) {
        if self.x != size.columns - 1 {
            self.x += 1;
        }
    }

    fn page_up(&mut self, size: &WindowSize) {
        let times = size.rows / 2;
        for _ in 0..times {
            self.move_up(size);
        }
    }

    fn page_down(&mut self, size: &WindowSize) {
        let times = size.rows / 2;
        for _ in 0..times {
            self.move_down(size);
        }
    }

    fn home(&mut self, _: &WindowSize) {
        self.x = 0;
    }

    fn end(&mut self, size: &WindowSize) {
        self.x = size.columns - 1;
    }

    fn should_move_up(&self, k: KeyEvent) -> bool {
        k.code == KeyCode::Char('w') && k.modifiers == KeyModifiers::NONE || k.code == KeyCode::Up
    }

    fn should_move_down(&self, k: KeyEvent) -> bool {
        k.code == KeyCode::Char('s') && k.modifiers == KeyModifiers::NONE || k.code == KeyCode::Down
    }

    fn should_move_left(&self, k: KeyEvent) -> bool {
        k.code == KeyCode::Char('a') && k.modifiers == KeyModifiers::NONE || k.code == KeyCode::Left
    }

    fn should_move_right(&self, k: KeyEvent) -> bool {
        k.code == KeyCode::Char('d') && k.modifiers == KeyModifiers::NONE
            || k.code == KeyCode::Right
    }

    fn should_page_up(&self, k: KeyEvent) -> bool {
        k.code == KeyCode::PageUp
    }

    fn should_page_down(&self, k: KeyEvent) -> bool {
        k.code == KeyCode::PageDown
    }

    fn should_home(&self, k: KeyEvent) -> bool {
        k.code == KeyCode::Home
    }

    fn should_end(&self, k: KeyEvent) -> bool {
        k.code == KeyCode::End
    }
}
