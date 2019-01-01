use std::fmt;
use std::ops::Range;
use yalr::{Lexer, YALR};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ABC {
    A,
    B,
    End,
    Error,
}

impl fmt::Display for ABC {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

pub struct ABLexer<'source> {
    input: &'source [ABC],
    pos: usize,
}

impl<'source> ABLexer<'source> {
    pub fn new(input: &'source [ABC]) -> Self {
        Self { input, pos: 0 }
    }
}

impl<'source> Lexer<'source, ABC, &'source [ABC]> for ABLexer<'source> {
    const ERROR: ABC = ABC::Error;
    const END: ABC = ABC::End;

    fn advance(&mut self) {
        if self.pos < self.input.len() {
            self.pos += 1;
        }
    }

    fn terminal<'t, 'lexer: 't>(&'lexer self) -> &'t ABC {
        &self.input[self.pos]
    }

    #[allow(clippy::range_plus_one)]
    fn range(&self) -> Range<usize> {
        Range {
            start: self.pos,
            end: self.pos + 1,
        }
    }

    fn slice(&self) -> &'source [ABC] {
        &self.input[self.pos..=self.pos]
    }
}

pub struct ABParser;

impl<'source> YALR<'source> for ABParser {
    type Terminal = ABC;
    type Input = &'source [ABC];
    type Output = String;
}
