use std::{
    fmt::{self,Display},
    path::{Path,PathBuf,} //引入PathBuf结构体,用于表示文件路径,内部提供方法修改，是可变类型
};

///存储文件地址的数据结构
#[derive(Default,Debug,Clone)]
pub struct FileInfo {
    path: Option<PathBuf>, //PathBuf 能够很好地对文件地址进行一些操作，比如push,pop,join,set_extension
}

impl FileInfo {
    ///将字符串形式的地址转换为Pathbuf类型
    pub fn from(file_name: &str) -> Self { 
        Self {
            path: Some(PathBuf::from(file_name)),
        }       
    }

    ///获取Option<&Path>
    pub fn get_path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    ///判断是否有地址
    pub const fn has_path(&self) -> bool {
        self.path.is_some()
    }

}

impl Display for FileInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //如果path不为空，则会返回路径;如果为空，则会返回"[No name]"
        let name = self.get_path()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("[No Name]");

        write!(f,"{name}")     
    } 
}