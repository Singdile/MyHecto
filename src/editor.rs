mod terminal;
mod view;
mod editorcommand;

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
///save the caret position
#[derive(Default)]
struct Location {
    x: usize,
    y: usize,
}
#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View
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
        let mut view = View::default();
        let args:Vec<String> = env::args().collect();

        if let Some(path) = args.get(1) {  //如果有传入文件参数，则更新buffer
            view.load(path);
        }

        Ok( Self {
            should_quit:false,
            view
        })
        

    }

    pub fn run(&mut self) {
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
                    self.view.handle_command(command);
                }
            },
            Err(err) => {}
        }
    } 
    



    fn refresh_screen(&mut self) {//刷新屏幕，这里忽略了对error的处理，即使发生也只是光标是否可见的问题
        let _ = Terminal::hide_caret();
        self.view.render();//对整个屏幕渲染

        let _ = Terminal::move_caret_to(self.view.caret_position());

        let _ = Terminal::show_caret();
        let _ = Terminal::execute();

    }


}

