use std::{cmp::min,io::Error};
use crate::editor::terminal::{Size, Terminal};
use super::{UIComponent};
use super::line::Line;
use crate::editor::command::{Edit};

///展现对未创建的文件的指令显示
pub struct CommandBar {
    prompt: String,
    value: Line,
    needs_redraw: bool,
    size: Size,
}


impl CommandBar {
   ///处理对commandbar的编辑指令
   pub fn handle_edit_command(&mut self, command: Edit ) {
        match command {
           Edit::Insert(ch) => {self.value.append_char(ch);},
           Edit::InsertNewline => {},
           Edit::Delete => {},
           Edit::DeleteBackward => { self.value.delete_last();},
        }

        self.set_needs_redraw(true);
   } 

   pub fn caret_position_end(&self) -> usize {
        let max_width = self.prompt.len().saturating_add(self.value.grapheme_count());
        min(max_width, self.size.columns)
   }

   pub fn value(&self) -> String {
        self.value.to_string()
   }

   pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_string()
   }
}

impl UIComponent for CommandBar {
    fn set_needs_redraw(&mut self,value:bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
       self.needs_redraw 
    }

    fn set_size(&mut self,size: Size) {
        self.size = size;
    }

    fn draw(&mut self, position_row:usize) -> Result<(),std::io::Error> {
        //prompt之后的长度
       let area_for_value = self.size.columns.saturating_sub(self.prompt.len()); 

        //value的视觉长度
       let value_end = self.value.width();

       //print value out from value_start
       let value_start =  value_end.saturating_sub(area_for_value);

       //用于打印的信息
        let message = format!("{}{}",self.prompt,self.value.get_visible_graheme(value_start..value_end));

        let to_print = if message.len() <= self.size.columns {
            message
        } else {
            String::new()
        };
        
        Terminal::print_row(position_row, &to_print)

    }
    
}