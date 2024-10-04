use std::fmt;

#[derive(Debug)]
pub struct EncodeError(pub String);


impl<'a> fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for EncodeError {}


#[derive(Debug)]
pub struct ParseError(pub String);


impl<'a> fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParseError {}
