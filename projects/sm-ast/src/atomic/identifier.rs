use super::*;

/// Represents a namespace, which is a sequence of identifiers.
#[derive(Debug, Clone, Eq, Hash)]
pub struct Identifier {
    pub name: ArcStr,
    pub span: Range<usize>,
    pub file: ArcStr,
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}
impl Ord for Identifier {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl Identifier {
    pub fn new(name: ArcStr) -> Self {
        Self { name, span: Default::default(), file: Default::default() }
    }
    pub fn with_file(self, file: ArcStr) -> Self {
        Self { name: self.name.clone(), span: self.span.clone(), file }
    }
    pub fn with_span(self, span: Range<usize>) -> Self {
        Self { name: self.name.clone(), span, file: self.file.clone() }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
