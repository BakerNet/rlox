use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceLocation {
    line: usize,
    pos: usize,
}

impl SourceLocation {
    pub fn new(line: usize, pos: usize) -> Self {
        Self { line, pos }
    }

    pub fn advance_by(&mut self, count: usize) {
        self.pos += count;
    }

    pub fn newline(&mut self) {
        self.line += 1;
        self.pos = 0;
    }

    pub fn merge(&mut self, other: SourceLocation) {
        self.line += other.line;
        if other.line > 0 {
            self.pos = other.pos;
        } else {
            self.pos += other.pos;
        }
    }
}

impl Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}:{}", self.line, self.pos)
    }
}
