use std::result;

use super::{
    terminal::{Size,Terminal},
    DocumentStatus,   
};


pub struct StatusBar {
    current_status: DocumentStatus,
    needs_redraw: bool,
    margin_bottom: usize,//与最后一行的距离
    width: usize,
    position_rows: usize //实际位于的行数,通常是倒数一二行
}

impl StatusBar {
    ///初始化statusBar
    pub fn new(margin_bottom: usize) -> Self {
        let size = Terminal::size().unwrap_or_default();

        Self {
            current_status: DocumentStatus::default(),
            needs_redraw: true,
            margin_bottom,
            width:size.columns,
            position_rows: size.rows.saturating_sub(margin_bottom).saturating_add(1),
        }
    }

    ///当terminal的size更新的时候，更新statusBar
    pub fn resize(&mut self, size: Size) {
        self.width = size.columns;
        self.position_rows = size.rows.saturating_sub(self.margin_bottom).saturating_add(1);
        self.needs_redraw = true;
    }

    ///当statusbar包含的信息发生改变的时候，更新statusbar
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if new_status != self.current_status {
           self.current_status = new_status;
           self.needs_redraw = true; 
        }
    }

    ///渲染statusbar
    pub fn render(&mut self) {

        let mut status = format!("{:?}",self.current_status);
        status.truncate(self.width);
        let result = Terminal::print_row(self.position_rows, &status);
        self.needs_redraw = false;
    }
}