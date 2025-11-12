
///记录文件状态的结构体
#[derive(Default,Eq,PartialEq,Debug)]
pub struct DocumentStatus {
    total_lines:usize,
    current_line_index: usize,
    is_modified: bool,
    file_name: Option<String>
}


impl DocumentStatus {
    ///创建一个新的DocumentStatus
    pub fn new(total_lines:usize,current_line_index:usize,is_modified:bool,file_name:Option<String>) -> Self {
        Self {
            total_lines,
            current_line_index,
            is_modified,
            file_name
        }
    }

    ///展示当前文件是否被修改的指示符
    pub fn modified_indicator_to_string(&self) -> String {
        if self.is_modified {
            String::from("(modified)")
        } else {
            String::new()
        }
    }

    ///展示当前文件的全部行号
    pub fn line_count_to_string(&self) -> String {
        format!("{} lines",self.total_lines)
    }

    ///当前所在行号
    pub fn position_indicator_to_string(&self) -> String {
        format!("{}/{}",self.current_line_index + 1,self.total_lines)
    }

    
}