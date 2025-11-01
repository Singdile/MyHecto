mod buffer;
mod line;

use std::cmp::min;

use super::editorcommand::{Direction,EditorCommand};
use super::terminal::{Size, Terminal};
use crate::editor::terminal::Position;
use crate::editor::view::buffer::Buffer;

use unicode_segmentation::UnicodeSegmentation;
use crate::editor::view::line::Line;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
#[derive(Copy, Clone,Default)]
pub struct Location {
    pub grapheme_index:usize,
    pub line_index:usize,
}

///理解可视窗口的大小比较重要：正如终端的尺寸在缩小或扩展之后是一定的。
/// scroll_offset 表示从文本的第几行开始渲染，是一个偏移量。scroll_offset = 1, 则会从 文本的第二行开始渲染(起始行号为0),直到铺满屏幕
/// 很多时候，文本的大小不能在一定尺寸的可视终端窗口大小中完全看完，当光标不断变化，直到光标超出当前可视窗口的渲染的范围的时候；则需要改变渲染的偏移量，让光标想要查看的位置重新渲染。
/// 举例 scroll_offset = 0 ; size = (2,2) ; 起始光标位置 (0,0)。 此时光标位置想要查看的是文本中绝对位置的第0行，恰好处理可视范围文本的(0~1)之内。
/// 光标移动到(2,0),想要查看第2行，超出了渲染的范围(0~1); 更改渲染的偏移量scroll_offset = 1,可视文本范围(1~2)
pub struct View {
    buffer: Buffer,
    need_redraw: bool,
    size: Size,
    text_location: Location,//文本中的第几行的第几个 grapheme
    scroll_offset: Position,//物理屏幕上的行列
    // location: Location,  //文本的绝对行数
    // scroll_offset: Location //从文本的第几行开始渲染
}


impl View {
    /// 负责打印输出内容，以及空行
    pub fn render(&mut self) {
        if !self.need_redraw {
            return
        }


        let Size {columns,rows} = self.size;
        if columns == 0 || rows == 0 {
            return
        }

        let vertical_center = columns/3;
        let top = self.scroll_offset.row; //可以显示的文本起始行

        for current_row in 0..rows {
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.column; //可以显示的文本起始列
                let right = self.scroll_offset.column.saturating_add(columns);//可以显示的文本终止列的下一列
                Self::render_line(current_row, &line.get_visible_graheme(left..right)); //获取可视范围的文本,渲染
            } else if current_row == vertical_center && self.buffer.is_empty() {//缓冲区没有内容，需要输出欢迎信息
                Self::render_line(current_row, &Self::build_welcome_message(columns));
            } else {//输出空行
                Self::render_line(current_row, "~");
            }
        }

        self.need_redraw = false;//渲染之后，将need_redraw重置
    }


    ///窗口大小变化的时候，需要重新绘制
    pub fn resize(&mut self,to: Size) {
        self.size = to;
        self.scroll_text_location_into_view();
        self.need_redraw = true;
    }


    ///渲染一行
    pub fn render_line(row: usize, line_text: &str) {
        let result = Terminal::print_row(row, line_text);
        debug_assert!(result.is_ok(),"Failed to render line");//这里对渲染进行了错误处理，上层render不用处理这里的错误
    }


    pub fn build_welcome_message(columns:usize) -> String {
        if columns == 0 {
            return "".to_string();
        }
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        if columns < welcome_message.len() {
            return "~".to_string();
        }

        let message_len = welcome_message.len();
        let padding_len = columns.saturating_sub(message_len) / 2;
        let padding = " ".repeat(padding_len.saturating_add(1) as usize);//填充的空格

        welcome_message = format!("~{padding}{welcome_message}");
        welcome_message.truncate(columns);
        welcome_message
    }
    ///处理键入事件
    pub fn handle_command(&mut self,command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Quit => {},
            EditorCommand::Insert(ch) => self.insert_char(ch),
            EditorCommand::Delete => { self.delete()},
            EditorCommand::Backspace => {self.backspace()},
            EditorCommand::Tab => { self.tab()} 
        }
    }

    ///处理按键Tab
    fn tab(&mut self) {
        //处理Tab为，插入\t
        self.insert_char('\t');
        self.need_redraw = true;
    }

    ///执行delete,删除光标后面的一位字符
    fn delete(&mut self) {
        //光标位置后面有一位
        self.buffer.delete(self.text_location);
        self.need_redraw = true;
    }

    ///执行backspace,删除光标前面的一个字符
    fn backspace(&mut self) {
        //光标位置前面有一位
        self.buffer.backspace(self.text_location);
        //光标向前移一位
        self.move_left();

        self.need_redraw = true;
    }

    ///插入字符
    fn insert_char(&mut self,ch: char) {
        let old_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| line.grapheme_count());
        self.buffer.insert_char(ch,self.text_location);
        let new_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0,Line::grapheme_count);

        let grapheme_delta = new_len.saturating_sub(old_len);
        if grapheme_delta > 0 {//增添后光标向右移动1位，即插入字符之
            self.move_right();
        }
        self.need_redraw = true;
    }
    ///移动想要显示的文本的绝对位置Location,文本中的第几行的第几个grapheme
   pub fn move_text_location(&mut self,direction:&Direction) {//处理移动指令，修改想要显示的文本的绝对位置
        match direction {
            Direction::Up => {
                self.move_up(1);
            },
            Direction::Down => {
                self.move_down(1);
            },
            Direction::Left => {
                self.move_left();
            },
            Direction::Right => {
                self.move_right();
            },
            Direction::Pageup => {
                //向上翻滚一页的内容,减一是保证最后一行的内容是翻滚之前的，对用户更加友好
               self.move_up(self.size.rows.saturating_sub(1));
            },
            Direction::PageDown => {
                //向下翻滚一页的内容,减一是保证最后一行的内容是翻滚之前的，对用户更加友好
                self.move_down(self.size.rows.saturating_sub(1));
            },
            Direction::Home => {
                self.move_to_start_of_line();
            },
            Direction::End => {
                self.move_to_end_of_line();
            }
        }
        self.scroll_text_location_into_view();

   }

    ///移动Location,向上一行
   fn move_up(&mut self,step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_line();
    }

    ///移动Location，向下一行
    fn move_down(&mut self,step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }
    ///移动Location,向左移动多少的grapheme
    fn move_left(&mut self) {
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index -= 1;
        } else if self.text_location.line_index > 0{
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }
    ///移动Location,向右移动1个grapheme
    fn move_right(&mut self) {
        let line_grapheme_len = self.buffer.lines.get(self.text_location.line_index).map_or(0, |v| v.grapheme_count());
        if self.text_location.grapheme_index < line_grapheme_len {
            self.text_location.grapheme_index += 1;
        } else {
            self.move_down(1);
            self.move_to_start_of_line();
        }
    }

    ///将Location的位置移动到一行的末尾,即一行的grapheme个数
    fn move_to_end_of_line(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| { line.grapheme_count() });
    }

    ///将Location的位置移动到一行的开头
    fn move_to_start_of_line(&mut self) {
        self.text_location.grapheme_index = 0;
    }

    /// 位置规范，保证text_location.grapheme_index的位置合法
   fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| {
                min(line.grapheme_count(), self.text_location.grapheme_index)
            });
   }
    /// 位置规范，保证text_location.line_index的位置合法
    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = min(self.text_location.line_index,self.buffer.height());
    }
    ///修改可视范围，使得光标所在行在屏幕的行可视范围内
   fn scroll_vertically(&mut self,to: usize) {
        let Size {columns:_,rows} = self.size;
        let offset_changed = if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            true
        } else if to >= self.scroll_offset.row.saturating_add(rows) {
            self.scroll_offset.row = to.saturating_sub(rows).saturating_add(1);
            true
        } else {
            false
        };

        self.need_redraw = self.need_redraw || offset_changed;
   }

   ///修改可视范围，使得光标所在列在屏幕的列可视范围内
   pub fn scroll_horizontally(&mut self,to: usize) {
        let Size {columns,..} = self.size;
        let offset_changed = if to < self.scroll_offset.column {
            self.scroll_offset.column = to;
            true
        } else if to >= self.scroll_offset.column.saturating_add(columns) {
            self.scroll_offset.column = to.saturating_sub(columns).saturating_add(1);
            true
        } else {
            false
        };
        self.need_redraw = self.need_redraw || offset_changed;
   }

   ///修改可视范围，使得光标所在行、列在屏幕的可视范围
   fn scroll_text_location_into_view(&mut self) {
        let Position { column, row } = self.text_locaton_to_position();
        self.scroll_vertically(row);
        self.scroll_horizontally(column);
   }

   ///转换location to Position
   fn text_locaton_to_position(&self) -> Position {
        let row = self.text_location.line_index;
        let column = self.buffer.lines.get(row).map_or(0, |line|
            line.width_until(self.text_location.grapheme_index)
        );
        Position { column, row }
   }
    /// 返回渲染在终端屏幕上的的绝对位置。比如屏幕左上角偏移显示的是第20行，text_location是文本的第50行
    /// 则第50行应该渲染在屏幕的第30行
    pub fn caret_position(&self) -> Position {
        self.text_locaton_to_position().saturating_sub(self.scroll_offset)
    }
    ///将对应路径文件，加载到buffer
    pub fn load(&mut self, path: &str) {
        if let Ok(buffer) = Buffer::load(path) {
            self.buffer = buffer;
            self.need_redraw = true;
        }
    }
}



impl Default for View {
    fn default() -> Self {
        View {
            buffer: Buffer::default(),
            need_redraw: true,
            size: Size::default(),
            text_location: Location::default(),
            scroll_offset: Position::default(),
        }
    }
}