
use std::result;

use crate::editor::{statusbar, terminal::Position};

use super::{
    terminal::{Size,Terminal},
    documentstatus::DocumentStatus,   
};

///代表状态栏的信息的结构体
pub struct StatusBar {
    current_status: DocumentStatus, //记录文件当前状态即状态栏的显示信息

    //下面的信息主要表示状态栏的显示位置
    width: usize,//状态栏的宽度
    position_rows: usize, //状态栏实际位于终端的行数
    margin_bottom: usize,//表示终端预留底部几行用于状态栏

    //是否可见与渲染
    is_visible: bool, //状态栏是否可见
    needs_redraw: bool, //是否需要重新渲染

}

impl StatusBar {
    ///初始化statusBar,margin_bottom表示位于倒数第几行
    /// 初始化状态栏的位置，但是状态栏的显示信息还没有初始化
    pub fn new(margin_bottom: usize) -> Self {
        let size = Terminal::size().unwrap_or_default();
        let mut statusbar = Self {
            current_status: DocumentStatus::default(), //初始化为空
            margin_bottom,
            needs_redraw: true,
            width: size.columns,
            position_rows:0,
            is_visible:false,//初始化设置为不可见
        };

        //具体由终端size,更新状态栏的位置参数,设置是否可见
        statusbar.resize(size);

        statusbar

    }

    ///当terminal的size更新的时候，更新statusBar的位置参数,并设置状态栏是否可见
    pub fn resize(&mut self, size: Size) {
        self.width = size.columns;
        let mut position_row = 0;
        let mut is_visible = false;

        if let Some(result) = size.rows
                .checked_sub(self.margin_bottom)
                .and_then(|result| Some(result)) 
        {
            position_row = result;
            is_visible = true; //状态设置为可见
        }

        self.position_rows = position_row;
        self.is_visible = is_visible;
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
        //检查是否满足渲染条件
        if !self.needs_redraw || !self.is_visible {
            return;
        }
        
        //hecto.rs - 23 lines (modified)       2/23
        if let Ok(size) = Terminal::size() {
            //拼装组合起，显示的信息
            let line_count = self.current_status.line_count_to_string();
            let modified_indicator = self.current_status.modified_indicator_to_string();

            let beginning = format!("{} - {} {}",self.current_status.file_name, line_count, modified_indicator);

            let position_indicator = self.current_status.position_indicator_to_string();

            //显示的信息
            let right_len_left = self.width.strict_sub(beginning.len());
            let status_info = format!("{beginning}{position_indicator:>right_len_left$}");
            
            //打印
           let to_print = if status_info.len() <= size.columns {
                status_info
           } else {
               String::new()
           };

           let result = Terminal::print_row(self.position_rows, &to_print);
           self.needs_redraw = false; 
        }
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