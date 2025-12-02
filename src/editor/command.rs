use crossterm::event::{
    self, Event, KeyCode::{Backspace, Delete, Down, End, Enter, Home, Left, PageDown, PageUp, Right, Tab, Up, char}, KeyEvent, KeyModifiers
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

pub enum Edit {
   Insert(char) 
}