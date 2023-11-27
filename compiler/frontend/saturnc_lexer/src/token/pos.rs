#[derive(Clone, Copy, Debug)]
pub struct Pos {
    line: usize,
    column: usize,
    start: usize,
}

impl Pos {
    pub fn get_line(self) -> usize {
        self.line
    }

    pub fn get_column(self) -> usize {
        self.column
    }

    /// Get lexeme end index
    pub fn get_end(self) -> usize {
        self.start + self.column
    }
}

