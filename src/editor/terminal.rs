use crossterm::cursor::{MoveTo,Hide,Show};
use crossterm::style::Print;
use crossterm::{queue, Command};
use crossterm::terminal::{disable_raw_mode,enable_raw_mode,size,Clear,ClearType,EnterAlternateScreen,LeaveAlternateScreen};
use std::io::{stdout,Write,Error};
pub struct Terminal {}
#[derive(Clone, Copy)]
pub struct Size { 
    pub columns:usize,
    pub rows: usize
}

#[derive(Clone, Copy,Default)]
pub struct Position {
    pub column:usize,
    pub row:usize 
}

impl Default for Size {
    fn default() -> Self {
        let Size{columns,rows} = Terminal::size().unwrap();
        Self { columns, rows }
    }
}


impl Position {
    pub const fn saturating_sub(self,other:Self) -> Self {
        Self {
            column: self.column.saturating_sub(other.column),
            row: self.row.saturating_sub(other.row)
        }
    }
}

/// 不同平台上的usize大小不同
/// 在 `usize < 16`的平台上,`u16::MAX`可能超出usize的范围
/// 解决方法:返回的尺寸大小限制在 `usize::MAX` 和 `u16::MAX`中较小的值的范围内
impl Terminal {

}

impl Terminal {
    pub fn terminate() -> Result<(),std::io::Error> {
        Self::leave_alternate_screen()?; //结束的时候，离开副屏幕
        Self::show_caret()?;
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn initialize() -> Result<(),std::io::Error> {
        enable_raw_mode()?;//开启终端的原始模式
        Self::enter_alternate_screen()?;//进入副屏幕
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    ///离开副屏幕指令
    pub fn leave_alternate_screen() -> Result<(),Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    } 

    ///进入副屏幕指令
    pub fn enter_alternate_screen() -> Result<(),Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(),std::io::Error> {
        Self::queue_command(Clear(ClearType::All))?; //执行终端清除，指令
        Ok(())
    }

    pub fn clear_line() -> Result<(),Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    ///移动光标到指定的位置
    ///当指定位置超出了`u16::MAX`范围，会被截断
    pub fn move_caret_to(position: Position) -> Result<(),std::io::Error> {
        #[allow(clippy::as_conversions,clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.column as u16,position.row as u16))?;
        Ok(())
    }

    ///返回终端的`size`
    ///任何一个坐标值都会截断在 `usize` 范围内,当 `usize < u16`
    pub fn size() -> Result<Size,std::io::Error> {
        let (columns,rows) = size()?;
        let columns = columns as usize;
        let rows = rows as usize;
        Ok(Size {columns,rows})
    }

    pub fn hide_caret() -> Result<(),std::io::Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_caret() -> Result<(),Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    
    pub fn print(string: &str) -> Result<(),Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    ///光标移动到指定行,打印指定的行信息
    pub fn print_row(row:usize,line_text:&str) -> Result<(),Error> {
        Self::move_caret_to(Position {column:0,row })?;
        Self::clear_line()?;
        Self::print(line_text)?;

        Ok(())
    }

    pub fn execute() -> Result<(),Error> {//确保写入的信息、命令执行
        //queue! 会将命令行操作，输入缓冲区队列；execute! 会将命令行操作输入缓冲区队列，并立即将缓冲区清空(即flush)
        stdout().flush()?;//理解将缓冲区的信息输出
        Ok(())
    } 

    pub fn queue_command<T:Command>(command:T) -> Result<(),Error> {
        queue!(stdout(),command)
    }
}