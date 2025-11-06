use core::default::Default;
use std::fs::read_to_string;
use std::io::Error;
use super::Location;
use super::line::Line;
#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>
}


impl Buffer {
    ///当仅判断是否存在的时候，使用bool
    ///当判断是否存在并且要返回值的时Option<>
   pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
   } 

   ///每次传入文件都是全新的buffer
   pub fn load(file_name: &str) -> Result<Self,Error> {
        let contents = read_to_string(file_name)?; 
        let mut content_lines = Vec::new();

        for line in contents.lines() {
           content_lines.push(Line::from(line)); 
        }        
        Ok(Self { lines: content_lines })
    }

    ///文本的行数
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    ///插入字符
    pub fn insert_char(&mut self,character: char, at: Location) {
        if at.line_index > self.height() {
            return;
        }
        if at.line_index == self.height() {
            self.lines.push(Line::from(&character.to_string()));
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(character,at.grapheme_index);
        }

    }

    ///delete,删除光标后面的一位字符
    pub fn delete(&mut self, at: Location) {
        if let Some(line) = self.lines.get(at.line_index) {
            //判断是不是最后一个
            if at.grapheme_index >= line.grapheme_count() 
                && at.line_index.saturating_add(1) < self.lines.len() {
                    let next_line = self.lines.remove(at.line_index.saturating_add(1));
                    self.lines[at.line_index].append(&next_line);
            } else if at.grapheme_index < line.grapheme_count() {//不是最后一个，直接删除光标所在位置的字符
                self.lines[at.line_index].delete(at.grapheme_index);
            }
        }
    }
    ///backspace,删除光标前面的一位字符
    pub fn backspace(&mut self, at: Location) {
        if let Some(line) = self.lines.get_mut(at.line_index) {
            if at.line_index == 0 && at.grapheme_index == 0 {
                //do nothing
            } else if at.line_index != 0 && at.grapheme_index == 0 {//光标位于一行的开头,转换为delete上一行的末尾
                let previous_line = at.line_index.saturating_sub(1);

                let previous_index = if let Some(line) = self.lines.get(previous_line) {
                    line.grapheme_count()
                } else {
                    0
                };

                let previous_positon = Location {
                    line_index: previous_line,
                    grapheme_index: previous_index
                };
               self.delete(previous_positon);
            } else {//正常情况下，删除
                line.delete(at.grapheme_index.saturating_sub(1));
            }
        }
    }

    ///处理tab按键
    pub fn tab(&mut self,at:Location) {
        if let Some(line) = self.lines.get_mut(at.line_index) {
            let new_line = line.split_off(at.grapheme_index);
            self.lines.insert(at.line_index.saturating_add(1), new_line);
        }
    }
}