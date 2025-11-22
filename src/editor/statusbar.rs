
use std::result;

use super::{
    terminal::{Size,Terminal},
    documentstatus::DocumentStatus,   
};

///代表状态栏的信息的结构体
pub struct StatusBar {
    current_status: DocumentStatus, //记录文件当前状态即状态栏的显示信息
    //下面的信息主要表示状态栏的显示位置
    needs_redraw: bool, //是否需要重新渲染
    margin_bottom: usize,//表示终端预留底部几行用于状态栏
    width: usize,//状态栏的宽度
    position_rows: usize, //状态栏实际位于终端的行数
    // is_visible: bool, //状态栏是否可见
}

impl StatusBar {
    ///初始化statusBar,margin_bottom表示位于倒数第几行
    pub fn new(margin_bottom: usize) -> Self {
        let size = Terminal::size().unwrap_or_default();

        Self { 
            current_status: DocumentStatus::default(),
            needs_redraw: true,
            margin_bottom,
            width: size.columns,
            //绘制在预留空间的第一行
            position_rows: size.rows.saturating_sub(margin_bottom)
        }
    }

    ///当terminal的size更新的时候，更新statusBar的位置参数
    pub fn resize(&mut self, size: Size) {
        self.width = size.columns;
        self.position_rows = size.rows.saturating_sub(self.margin_bottom);
        self.needs_redraw = true;
    }

    ///当statusbar对应的信息documentstatus改变，更新status
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if new_status != self.current_status {
           self.current_status = new_status;
           self.needs_redraw = true; 
        }
    }
    ///渲染statusbar
    pub fn render(&mut self) {
        if !self.needs_redraw { //判断是否需要渲染
            return;
        }

        let mut status = format!("{:?}",self.current_status);
        status.truncate(self.width);  
        //执行渲染
        let result = Terminal::print_row(self.position_rows, &status);
        self.needs_redraw = false;
    }
}



#[cfg(test)]
mod tests {

    use super::*;
    use super::Terminal;

    #[test]
    fn test_statusbar_new() {
        let statusbar = StatusBar::new(2);
        let size = Terminal::size().unwrap();        
        println!("{}",size.rows);
        assert_eq!(statusbar.position_rows,size.rows-2)

    }
}