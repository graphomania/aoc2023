use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub struct ParseError {
    what: String,
}

impl ParseError {
    pub fn from<T: ToString>(s: &T) -> ParseError {
        Self {
            what: s.to_string(),
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        ParseError::from(&value.to_string())
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse error failed {}", self.what)
    }
}