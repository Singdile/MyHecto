use std::{
    fmt::{self,Display},
    path::PathBuf, //引入PathBuf结构体,用于表示文件路径,内部提供方法修改，是可变类型
};

///存储文件地址的数据结构
#[derive(Default,Debug,Clone)]
pub struct FileInfo {
    pub path: Option<PathBuf>, //PathBuf 能够很好地对文件地址进行一些操作，比如push,pop,join,set_extension
}

impl FileInfo {
    pub fn from(file_name: &str) -> Self { 
        Self {
            path: Some(PathBuf::from(file_name)),
        }       
    }
}

impl Display for FileInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //如果path不为空，则会返回路径;如果为空，则会返回"[No name]"
        let name = self 
            .path
            .as_ref()
            .and_then(|name| name.to_str()) //and_then(),如果是Some(T)，T传递给函数;否则，直接返回None
            .unwrap_or("[No name]");

        write!(f,"{name}")     
    } 
}