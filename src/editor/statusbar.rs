use std::io::Error;

use super::{
    terminal::{Size,Terminal},
    documentstatus::DocumentStatus,   
    uicomponent::UIComponent,
};

///代表状态栏的信息的结构体
#[derive(Default)]
pub struct StatusBar {
    current_status: DocumentStatus, //记录文件当前状态即状态栏的显示信息
    needs_redraw:bool,
    size: Size,
}

impl StatusBar {
    ///更新状态栏的文件信息
    pub fn update_status(&mut self,new_status: DocumentStatus) {
       if new_status != self.current_status {
            self.current_status = new_status;
            self.mark_redraw(true);
       } 
    }
}

impl UIComponent for StatusBar {
   fn mark_redraw(&mut self,value:bool) {
       self.needs_redraw = value
   } 

   fn needs_redraw(&self) -> bool {
       self.needs_redraw
   }

   fn set_size(&mut self,size: Size) {
        self.size = size       
   }

   fn draw(&self, position_row:usize) -> Result<(),Error> {
        let line_count = self.current_status.line_count_to_string();
        let modified_indicator = self.current_status.modified_indicator_to_string();

        let beginning = format!("{} - {line_count} {modified_indicator}", &self.current_status.file_name);   

        let position_indicator = self.current_status.position_indicator_to_string();
        let right_left = self.size.columns.saturating_sub(beginning.len());
        let status = format!("{beginning}{position_indicator:>right_left$}");

        let to_print = if status.len() <= self.size.columns {
            status
        } else {
            String::new()
        }; 

        Terminal::print_inverted_color_row(position_row, &to_print)
   }
}