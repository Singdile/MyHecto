use crossterm::event::{Event,KeyCode,KeyEvent,KeyModifiers,KeyEventKind};
use std::{convert::TryFrom};

use super::terminal::Size;


pub enum Direction {
    Pageup,
    PageDown,
    Home,
    End,
    Up,
    Left,
    Right,
    Down,
}

pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Insert(char),
    Delete,
    Backspace,
    Tab,
    Quit
}


///实现将Event事件，转换为EditorCommand；从而实现对ctrl+c 和 光标方向移动的操作
impl TryFrom<Event> for EditorCommand {
    type Error = String;
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            //按键事件,转换 
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind:KeyEventKind::Press,
                ..
            }) => {
                match (code,modifiers) {
                    (KeyCode::Char('c'),KeyModifiers::CONTROL) => { Ok(Self::Quit)},
                    //按键移动光标的事件
                    (KeyCode::PageUp,_) => { Ok(Self::Move(Direction::Pageup))},
                    (KeyCode::PageDown,_) => { Ok(Self::Move(Direction::PageDown))},

                    (KeyCode::Home,_) => { Ok(Self::Move(Direction::Home))},
                    (KeyCode::End,_) => { Ok(Self::Move(Direction::End))},
                    (KeyCode::Up,_) => { Ok(Self::Move(Direction::Up))},
                    (KeyCode::Left,_) => { Ok(Self::Move(Direction::Left))},
                    (KeyCode::Right,_) => { Ok(Self::Move(Direction::Right))},
                    (KeyCode::Down,_) => { Ok(Self::Move(Direction::Down))},

                    //按键键入普通的字符
                    (KeyCode::Char(ch),KeyModifiers::NONE | KeyModifiers::SHIFT) => { Ok(Self::Insert(ch))}

                    //remove char
                    (KeyCode::Delete,_) => { Ok(Self::Delete)},
                    (KeyCode::Backspace,_) => { Ok(Self::Backspace)}

                    //键入TAB键
                    (KeyCode::Tab,_) => { Ok(Self::Tab)}

                    _ => { Err(format!("keycode not surpported: {code:?}"))}

                }

            }

            //size 更改事件,转换
            Event::Resize(columns,rows ) => {
                let columns = columns as usize;
                let rows = rows as usize;

                Ok(Self::Resize(Size { columns, rows }))
            }
            _ => { Err(format!("Event not surpported: {event:?}"))}
        }        


    }

}