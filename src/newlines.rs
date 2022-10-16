struct UnescapedString<'a> {
    s: std::str::Chars<'a>,
}

impl<'a> UnescapedString<'a> {
    fn new(s: &'a str) -> Self {
        Self { s: s.chars() }
    }
}

impl Iterator for UnescapedString<'_> {
    type Item = Result<char, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.s.next().map(|c| match c {
            '\\' => match self.s.next() {
                None => Err(Error::EscapeAtEndOfString),
                Some('n') => Ok('\n'),
                Some('\\') => Ok('\\'),
                Some(c) => Err(Error::UnrecognizedEscapedChar(c)),
            },
            c => Ok(c),
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EscapeAtEndOfString,
    UnrecognizedEscapedChar(char),
}

use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::EscapeAtEndOfString => {
                write!(f, "Escape character at the end of the string")
            }
            Error::UnrecognizedEscapedChar(c) => {
                write!(f, "Unrecognized escaped char: '{}'", c)
            }
        }
    }
}

impl std::error::Error for Error {}

struct EscapedString<'a> {
    s: std::str::Chars<'a>,
}

impl<'a> EscapedString<'a> {
    fn new(s: &'a str) -> Self {
        Self { s: s.chars() }
    }
}

impl Iterator for EscapedString<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.s.next() {
            None => None,
            Some('\\') => Some(String::from("\\\\")),
            Some('\n') => Some(String::from("\\n")),
            Some(c) => Some(String::from(c)),
        }
    }
}

pub fn escape_string(s: &str) -> String {
    EscapedString::new(s).collect()
}

pub fn unescape_string(s: &str) -> Result<String, Error> {
    UnescapedString::new(s).collect()
}
