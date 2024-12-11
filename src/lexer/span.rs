use std::ops::Range;

use super::Location;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl Span {
    #[track_caller]
    pub const fn new(start: usize, end: usize) -> Self {
        debug_assert!(start <= end);
        Self { start, end }
    }

    pub const fn eof(end: usize) -> Self {
        Self { start: end, end }
    }

    pub const fn zero() -> Self {
        Self { start: 0, end: 0 }
    }
}

impl Span {
    pub fn consume(&mut self) {
        self.start = self.end;
    }

    pub const fn as_range(&self) -> Range<usize> {
        self.start..self.end
    }
}

impl Span {
    /// # Panics
    /// Panics if source does not contain span
    #[track_caller]
    pub fn location(&self, source: &str) -> Location {
        if source.len() < self.end {
            panic!("Span::location(source): source does not contain span!")
        }

        let mut line = 1;
        let mut column = 1;

        for (index, c) in source.chars().enumerate() {
            if index == self.start {
                break;
            }

            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += c.len_utf8() as u32;
            }
        }

        Location::new(line, column)
    }

    pub fn get_lexeme<'a>(&self, source: &'a str) -> Option<&'a str> {
        source.get(self.as_range())
    }

    pub fn lexeme<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..self.end]
    }

    pub fn len(&self) -> u32 {
        (self.end - self.start) as u32
    }
}
