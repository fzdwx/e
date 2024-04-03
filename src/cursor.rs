use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

pub struct Cursor {
    pub x: u16,
    pub y: u16,
}


impl Cursor {
    pub async fn react(&mut self, event: Event) -> Result<bool> {
        match event {
            Event::Key(k) => match k {
                KeyEvent { .. } => {
                    if self.should_move_up(k) {
                        self.move_up();
                    } else if self.should_move_down(k) {
                        self.move_down();
                    } else if self.should_move_left(k) {
                        self.move_left();
                    } else if self.should_move_right(k) {
                        self.move_right();
                    }
                }
            },
            _ => {}
        }

        Ok(false)
    }

    fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    fn move_down(&mut self) {
        self.y += 1;
    }

    fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    fn move_right(&mut self) {
        self.x += 1;
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
        k.code == KeyCode::Char('d') && k.modifiers == KeyModifiers::NONE || k.code == KeyCode::Right
    }
}