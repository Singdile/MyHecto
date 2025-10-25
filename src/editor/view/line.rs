
use std::{clone, ops::Range};
use crossterm::cursor::RestorePosition;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Copy)]
enum GraphemeWidth {
    Half,
    Full,
    Zero,
}

impl GraphemeWidth {
    const fn saturating_add(self,other: usize) -> usize {
        match self {
            Self::Zero => other.saturating_add(0),
            Self::Half => other.saturating_add(1),
            Self::Full => other.saturating_add(2),
        }
    }
}


struct TextFragment {
    grapheme: String,//字素簇
    rendered_width: GraphemeWidth,//视觉长度，有些字素簇看似是一个，但是需要占到2个位
    replacement: Option<char>,//处理不可见的字符，如非打印的控制字符\r\n
}
///数据结构，保存的是一行的grapheme数组
pub struct Line {
    fragments: Vec<TextFragment>,
}

impl Line {

    ///将一行字符串转换为一个数组,元素为TextFragment,记录了每个grapheme以及需要渲染的长度
    pub fn from(line_str:&str) -> Self {
        let fragments:Vec<TextFragment> = line_str
            .graphemes(true)
            .map(|grapheme| {
                let (replacement, rendered_width) = Self::replcement_character(grapheme)
                    .map_or_else(
                        || {
                            let unicode_width = grapheme.width();
                            let rendered_width = match unicode_width {
                                0 => GraphemeWidth::Zero,
                                1 => GraphemeWidth::Half,
                                _ => GraphemeWidth::Full,
                            };
                            (None, rendered_width)
                        },
                        |replacement| (Some(replacement), GraphemeWidth::Full),
                    );

                TextFragment {
                    grapheme: grapheme.to_string(),//字素簇，这里还是原封不动地保存解析出来的字符串
                    rendered_width,//通过处理判断渲染的长度
                    replacement,//通过处理判断是否需要替换
                }
            })
            .collect()
        ;
        Self { fragments }
    }
    ///判断每一个grapheme是否需要替换。
    /// 如果是空字符串和正常字符，返回None;
    /// 如果是特殊字符，则进行替换判断
    fn replcement_character(for_str: &str) -> Option<char> {
        let width = for_str.width();
        match for_str {
            " " => None, //空格
            "\t" => Some(' '),
            _ if width > 0 &&for_str.trim().is_empty() => Some('␣'),
            _ if for_str.width() == 0 => {//不可见字符
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next() {//如果是控制字符每，返回‘␣’
                    if ch.is_control() && chars.next().is_none() {
                        return Some('▯')
                    }
                }
                Some('·')  //非控制字符且不可见
            }
            _ => None,//正常的字符
        }
    }
    ///根据一个给定的视觉列范围，从一行完整的文本数据中，精确地提炼应该显示在屏幕上的一部分字符串
    /// range 表示可视宽度范围
    /// 正常能显示一整行，range指定了可视范围。光标从一行的开头遍历grapheme，直到光标移动到可视范围才收集字符用于打印
    pub fn get_visible_graheme(&self,range:Range<usize>) -> String {
        if range.start >= range.end {
            return String::new()
        }

        let mut result = String::new();
        let mut current_pos = 0;
        
        for fragment in &self.fragments {
            let fragment_end = fragment.rendered_width.saturating_add(current_pos);
            if current_pos >= range.end {//可视范围地末尾
                break;
            }
            
            if fragment_end > range.start {//判断是否可以显示
                if fragment_end > range.end || current_pos < range.start {//处理左右边界处的可视字符
                    result.push('⋯');
                } else if let Some(value) = fragment.replacement {
                    result.push(value);
                } else {
                    result.push_str(&fragment.grapheme);
                }
            }
            current_pos = fragment_end;//每次遍历一个grapheme就更新current_pos
        }
        result
    }

    ///grapheme的个数
    pub fn grapheme_count(&self) -> usize {
        self.fragments.len()
    }

    ///返回from 0 to grapheme_index,the visual length on terminal
    pub fn width_until(&self,grapheme_index:usize) -> usize {
        self.fragments
            .iter()
            .take(grapheme_index)
            .map(|fragment| match fragment.rendered_width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
                GraphemeWidth::Zero => 0,
            })
            .sum()
    }

    ///translate &str to Vec<TextFragment>
    // fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {
    //     line_str
    //         .graphemes(true),
    //         map(|grapheme| {
    //             let (replacement, rendered_width) = Self::replcement_character(grapheme)
    //                 .map_or_else(
    //                     || {
    //
    //                         (None,rendered_width)
    //                     }
    //                     ,
    //                 )
    //
    //         })
    // }
    ///在指定位置插入字符
    pub fn insert_char(&mut self,character: char, grapheme_index: usize) {
        let mut result= String::new();
        for (index,fragment) in self.fragments.iter().enumerate() {
            if index == grapheme_index {//指定位置，加入字符
                result.push_str(&character.to_string());
            }
            result.push_str(&fragment.grapheme);//非指定位置，直接加入
        }

        if grapheme_index >= self.fragments.len() {
           result.push_str(&character.to_string());
        }
        *self = Line::from(&result)
    }

    ///delete
    pub fn delete(&mut self,grapheme_index: usize) {
        let mut result = String::new();

        if grapheme_index >= self.grapheme_count() { return };

        if grapheme_index < self.grapheme_count() {
            for (index,fragment) in self.fragments.iter().enumerate() {
                if index != grapheme_index {
                    result.push_str(&fragment.grapheme);
                }
            }
        }
        *self = Line::from(&result);

    }
} 