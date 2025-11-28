
///代表简单信息的结构，比如键入ctr+s 显示 save
pub struct Messagebar {
    current_message: String, //当前的信息

    //下面的信息主要表示信息栏的显示位置
    width: usize,//状态栏的宽度
    position_rows: usize, //状态栏实际位于终端的行数
    margin_bottom: usize,//表示终端预留底部几行用于状态栏

    //是否可见与渲染
    is_visible: bool, //状态栏是否可见
    needs_redraw: bool, //是否需要重新渲染
}

impl Messagebar {
    ///初始化
}