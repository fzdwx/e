use anyhow::Result;
use ropey::{Rope, RopeBuilder};
use std::fmt::Display;
use std::path::Path;

pub struct Document {
    pub text: Rope,
}

impl Document {
    pub fn open(path: &Path) -> Result<Self> {
        let f = std::fs::File::open(path)?;
        let result = Rope::from_reader(f)?;

        Ok(Self { text: result })
    }

    pub fn get_lines(&self) -> usize {
        self.text.len_lines()
    }
}

impl From<&str> for Document {
    fn from(value: &str) -> Self {
        Self {
            text: Rope::from(value),
        }
    }
}
