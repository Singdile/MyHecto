use std::io::Error;
use crate::editor::terminal::Size;
use std::time::{Instant,Duration};
use super::uicomponent::UIComponent;
use super::Terminal;


const DEFAULT_DURATION: Duration = Duration::new(5, 0); //message_bar 显示时间
///用于Messagebar 内部，用于记录信息和时间点的数据结构
struct Message {
    text: String,
    time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self { 
            text: String::new(), 
            time: Instant::now(),
        }
    }
}

impl Message {
    ///判断信息是否过期
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
    
}


///代表简单信息的结构，比如键入ctr+s 显示 save
#[derive(Default)]
pub struct Messagebar {
    current_message: Message,
    needs_redraw: bool, //是否需要重新绘制
    cleared_after_expiry: bool, //记录信息过期后是否清除
}

impl Messagebar {
    pub fn update_message(&mut self, new_message: &str) {
        self.current_message = Message { 
            text: new_message.to_string(), 
            time: Instant::now()
        };
        self.cleared_after_expiry = false;
        self.set_needs_redraw(true);
    }
}


impl UIComponent for Messagebar {
    fn set_needs_redraw(&mut self,value:bool) {
        self.needs_redraw = value
    }

    fn needs_redraw(&self) -> bool {
        //明确需要渲染的时候是必须要渲染显示的
        //当没有明确要渲染的时候，判断信息是否过期; 若过期且未被清除，则需要重新渲染显示
        (self.current_message.is_expired() && !self.cleared_after_expiry) || self.needs_redraw 
    }

    fn set_size(&mut self,size: Size) {
       
   }


   fn render(&mut self,position_row:usize) {
        if self.needs_redraw() {
            match self.draw(position_row) {
                Ok(()) => self.set_needs_redraw(false),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("could not render component:{err:?}");
                    }
                }
            }
        }             
   }

    fn draw(&mut self, position_row:usize) -> Result<(),Error> {
        //需要渲染，但是信息过期,需要将原来的信息清除
        if self.current_message.is_expired() {
            self.cleared_after_expiry = true;
        }
        
        //打印的信息
        let message = if self.current_message.is_expired() {
            ""
        } else {
            &self.current_message.text
        };

        Terminal::print_row(position_row, message)
    }
}
