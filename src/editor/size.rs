use crate::editor::terminal::Terminal;
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Size { 
    pub columns:usize,
    pub rows: usize
}


impl Default for Size {
    fn default() -> Self {
        let Size{columns,rows} = Terminal::size().unwrap();
        Self { columns, rows }
    }
}

