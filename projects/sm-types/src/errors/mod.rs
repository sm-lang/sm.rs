use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

pub type Result<T> = std::result::Result<T, SmError>;

pub struct SmError {
    kind: Box<SmErrorKind>,
}

#[derive(Debug)]
pub enum SmErrorKind {
    Unknown,
}

impl Debug for SmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl Display for SmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl Error for SmError {}
