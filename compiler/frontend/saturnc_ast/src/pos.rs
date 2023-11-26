#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    line: usize,
    column: usize,
    line_start: usize,
}

impl Pos {
    pub fn line(self) -> usize {
        self.line
    }

    pub fn column(self) -> usize {
        self.column
    }

    pub fn line_start(self) -> usize {
        self.line_start
    }

    pub fn index(self) -> usize {
        self.line_start() + self.column() - 1
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            line_start: 0,
        }
    }
}
