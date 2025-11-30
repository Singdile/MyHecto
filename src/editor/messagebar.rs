use std::io::Error;
use crate::editor::terminal::Size;
use std::time::{Instant,Duration};
use super::uicomponent::UIComponent;
use super::Terminal;
///代表简单信息的结构，比如键入ctr+s 显示 save
#[derive(Debug)]
pub struct Messagebar {
    current_message: String,
    needs_redraw: bool,
    time_render: Instant, //自带一个计时点，并且该计时点只在更新内容的时候更新
}

impl Messagebar {
    pub fn update_message(&mut self, new_message: String) {
        if self.current_message != new_message {
            self.current_message = new_message;
            self.time_render = Instant::now(); //更新记录点
            self.mark_redraw(true);
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

   fn set_size(&mut self,size: Size) {
        
   }


   fn render(&mut self,position_row:usize) {
        let time_last = Instant::now() - self.time_render;
        if self.needs_redraw() {
            match self.draw(position_row) {
                Ok(()) => self.mark_redraw(false),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("could not render component:{err:?}");
                    }
                }
            }
        } else if time_last > Duration::new(5, 0) {
            Terminal::print_row(position_row, "~");
        }
   }

    fn draw(&self, position_row:usize) -> Result<(),Error> {
        Terminal::print_row(position_row, &self.current_message)
    }
}


impl Default for  Messagebar {
    fn default() -> Self {
        Self { 
            current_message: String::default(), 
            needs_redraw: false, 
            time_render: Instant::now(), 
        }
    }
}

#[cfg(test)]
mod tsets {
    use crate::editor::messagebar::Messagebar;
    use std::{thread, time::Duration};


    #[test]
    fn test_time() {
        let mut  mse = Messagebar::default();
        let first = mse.time_render;
        println!("time point:{:?}", first);
        thread::sleep(Duration::new(5,0));
        mse.update_message("hello".to_string()); 
        println!("time point:{:?}",mse.time_render);
        assert_eq!(true, mse.time_render - first <= Duration::new(6,0))

    }
}