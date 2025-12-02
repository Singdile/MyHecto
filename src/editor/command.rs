use crossterm::event::{
    self, Event, KeyCode::{self, Backspace, Delete, Down, End, Enter, Home, Left, PageDown, PageUp, Right, Tab, Up, char}, KeyEvent, KeyModifiers
};


use std::convert::TryFrom;

use super::terminal::Size;
///移动指令枚举
pub enum Move {
    Pageup,
    PageDown,
    StartofLine, //Home
    EndofLine, //End
    Up,
    Left,
    Right,
    Down,
}


impl TryFrom<KeyEvent> for Move {
//关联类型(Associated Types),表示一个类型(类型)，在特征中用于抽象一个类型,将定义交给实现者
    type Error = String; 


    //实现该特征，使得KeyEvent可以转换为Move类型
    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {

        let KeyEvent { 
            code,modifiers,..
        } = value;

        if modifiers == KeyModifiers::NONE {
            match code {
                PageUp => { Ok(Self::Pageup)},
                PageDown => {Ok(Self::PageDown)},

                Home => { Ok(Self::StartofLine)},
                End => { Ok(Self::EndofLine)},
                Up => { Ok(Self::Up)},
                Left => { Ok(Self::Left)},
                Right => { Ok(Self::Right)},
                Down => { Ok(Self::Down)},

                _ => { Err(format!("unsupported key code {code:?} or modifier {modifiers:?}" ))},
            }
        } else {
            Err(format!("unsupported key code {code:?} or modifier {modifiers:?}" ))
        }
    }
}

///编辑指令类型
pub enum Edit {
   Insert(char),
   InsertNewline,
   Delete,
   DeleteBackward, //Backsapce
}

impl TryFrom<KeyEvent> for Edit  {
    type Error = String; //指定装换失败的返回类型

    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent { 
            code,modifiers,..
        } = value;  //结构体解构赋值，这里前面是类型名，后面是变量名;快速获取，直接使用字段名作为变量名

        //匹配编辑操作
        match (code,modifiers) {
            (KeyCode::Char(ch),KeyModifiers::NONE | KeyModifiers::SHIFT) => { Ok(Edit::Insert(ch))},
            (KeyCode::Tab,KeyModifiers::NONE) => {Ok(Edit::Insert('\t'))},
            (KeyCode::Enter,KeyModifiers::NONE) => { Ok(Edit::InsertNewline)},

            (KeyCode::Delete,KeyModifiers::NONE) => {Ok(Edit::Delete)},
            (KeyCode::Backspace,KeyModifiers::NONE) => { Ok(Edit::DeleteBackward)},

            _ => {
                Err(format!("unsupported key code {code:?} or modifier {modifiers:?}" ))
            },
        }  
    }
}

///用于系统的指令，主要是终端显示的大小改变，保存文件，退出程序指令
pub enum System {
    Resize(Size),
    Save,
    Quit,
}

impl TryFrom<KeyEvent> for System  {
    type Error = String;
    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {

       let KeyEvent { code,modifiers,..}  = value;

       match (code,modifiers) {
           (KeyCode::Char('q'),KeyModifiers::CONTROL) => {Ok(System::Quit)},
           (KeyCode::Char('s'),KeyModifiers::CONTROL) => { Ok(System::Save)},
            _ => Err(format!("unsupported key code {code:?} or modifier {modifiers:?}")),
       } 

    }
}

///将设计的几种指令整合在一起，抽象
pub enum Command {
   Move(Move),
   Edit(Edit),
   System(System), 
}

///事件不仅涉及按键，还有屏幕尺寸的更改，所以这里的dy是Event
impl TryFrom<Event> for Command {
   type Error = String; 
   //todo
   fn try_from(value: Event) -> Result<Self, Self::Error> {
       
   } 
}