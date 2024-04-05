use std::borrow::Cow;
use std::fmt::Display;
use unicode_width::UnicodeWidthStr;

pub struct Row<'a> {
    pub content: Cow<'a, str>,
}

impl<'a> Row<'a> {
    pub fn width(&self) -> usize {
        self.content.width()
    }
}


impl<'a> From<&'a str> for Row<'a> {
    fn from(content: &'a str) -> Self {
        Self {
            content: content.into(),
        }
    }
}
