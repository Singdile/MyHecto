use std::io::Error;
use crate::editor::terminal::Size;

use super::uicomponent::UIComponent;
use super::Terminal;
///代表简单信息的结构，比如键入ctr+s 显示 save
#[derive(Default)]
pub struct Messagebar {
    current_message: String,
    needs_redraw: bool,
}

impl Messagebar {
    pub fn update_message(&mut self, new_message: String) {
        if self.current_message != new_message {
            self.current_message = new_message;
        }
    }
}


impl UIComponent for Messagebar {
   fn mark_redraw(&mut self,value:bool) {
        self.needs_redraw = value
   } 

   fn needs_redraw(&self) -> bool {
        self.needs_redraw
   }

   fn set_size(&mut self,size: Size) {}

    fn draw(&self, position_row:usize) -> Result<(),Error> {
        Terminal::print_row(position_row, &self.current_message)
    }
}