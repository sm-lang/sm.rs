use crate::Identifier;
use std::{cmp::Ordering, ops::Range};

/// Represents a namespace, which is a sequence of identifiers.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Symbol {
    pub namepath: Vec<Identifier>,
}

impl Symbol {
    pub fn new(parts: Vec<Identifier>) -> Self {
        Self { namepath: parts }
    }
    pub fn join(&self, part: Identifier) -> Self {
        let mut namepath = self.namepath.clone();
        namepath.push(part);
        Self { namepath }
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, part) in self.namepath.iter().enumerate() {
            if index == 0 {
                write!(f, "{}", part)?;
            }
            else {
                write!(f, "::{}", part)?;
            }
        }
        Ok(())
    }
}
