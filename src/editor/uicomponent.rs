use std::io::Error;
use super::terminal::Size;

pub trait UIComponent {
    ///标记是否需要渲染
    fn set_needs_redraw(&mut self,value:bool);

    ///查看是否需要渲染
    fn needs_redraw(&self) -> bool;

    ///当终端size更新的时候，同步更新组件的显示位置以及是否可见等视觉参数
    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.set_needs_redraw(true);
    }

    ///设置UI组件的显示size
    fn set_size(&mut self,size: Size);

    ///在指定行的位置渲染
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

    ///实际执行渲染的工具函数
    fn draw(&mut self, position_row:usize) -> Result<(),Error>; 
}