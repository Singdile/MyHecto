mod terminal;
mod view;
mod editorcommand;
mod statusbar;
mod documentstatus;
mod fileinfo;

use std::io::Error;
use std::panic::{set_hook,take_hook};
use std::env;
use crossterm::event::{
    Event,
    KeyEvent, KeyEventKind, read,
};
use terminal::{Terminal};
use view::View;


use crate::editor::editorcommand::EditorCommand;
use crate::editor::statusbar::StatusBar;
use crate::editor::terminal::Size;

const VERSION: &str = env!("CARGO_PKG_VERSION");//版本号
const NAME: &str = env!("CARGO_PKG_NAME");//文件名
///save the caret position
#[derive(Default)]
struct Location {
    x: usize,
    y: usize,
}




pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
}


impl Editor {
    ///初始化一个副屏幕，如果有合法的文件传入，则读取该文件到buffer
    pub fn new() -> Result<Self,Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {  //设置新的panic_hook,当panic的时候，会执行该panic_hook中的闭包函数
            let _ = Terminal::terminate();//离开副屏幕
            current_hook(panic_info);
        }));

        Terminal::initialize()?;//打开一个副屏幕

        //下面初始化Editor的元素
       let mut view = View::new(2);
       let args: Vec<String> = env::args().collect(); 
       if let Some(file_name) = args.get(1) { //第二个参数是传递进去的
            view.load(&file_name);
       }

       Ok(
            Self {
                should_quit: false,
                view,
                status_bar: StatusBar::new(2),
            }
       )

    }


    pub fn run(&mut self){ 
        loop {
            self.refresh_screen();

            if self.should_quit {
                // let _ = Terminal::terminate();//离开副屏幕
                break;
            }

            match read() {
                Ok(event) => {
                    self.evaluate_event(event);
                },
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }

            //处理事件之后，可能文本内容发生改变，即statsbar的DocumentStatus内容改变，判断是否需要修改
            //处理事件之后，可能终端的size大小发生变化，进而导致状态栏的显示位置变化，已在size变更指令处处理了该逻辑
            let status = self.view.get_status();
            self.status_bar.update_status(status);
        }
     
    }


   
    fn evaluate_event(&mut self, event: Event) {
        let should_process = match event {
            Event::Key(KeyEvent{kind,..}) => {
                kind == KeyEventKind::Press
            },
            Event::Resize(_,_ ) => {
                true
            },
            _ => false
        };
        
        if !should_process {
            return;
            // #[cfg(debug_assertions)]
            // {
            //     panic!("receive and discard unsupported or non-press event");
            // }
        }

        match editorcommand::EditorCommand::try_from(event) {
            Ok(command) => {
                if matches!(command,EditorCommand::Quit) {
                    self.should_quit = true;
                } else {
                    if let EditorCommand::Resize(size) = command {
                        //当终端的size变化的时候，更新状态栏的位置参数
                        self.status_bar.resize(size);
                    }
                    self.view.handle_command(command);

                }
            },
            Err(err) => {}
        }
    } 
    



    fn refresh_screen(&mut self) {//刷新屏幕，这里忽略了对error的处理，即使发生也只是光标是否可见的问题
        let _ = Terminal::hide_caret();
        self.view.render();//对整个屏幕渲染
        self.status_bar.render();//渲染状态栏
        let _ = Terminal::move_caret_to(self.view.caret_position());

        let _ = Terminal::show_caret();
        let _ = Terminal::execute();

    }


}

