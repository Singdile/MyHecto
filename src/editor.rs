mod terminal;
mod view;
mod statusbar;
mod documentstatus;
mod fileinfo;
mod messagebar;
mod uicomponent;
mod command;

use std::io::Error;
use std::panic::{set_hook,take_hook};
use std::{env};
use crossterm::event::{
    Event,
    KeyEvent, KeyEventKind, read,
};
use terminal::{Terminal};
use view::View;

use uicomponent::UIComponent;
use crate::editor::statusbar::StatusBar;

//self 表示当前的模块，editor;要使用下面的子模块，通常可以省略，为了路径清晰，可以添加上
use self::{
    command::{
        Command::{self,Edit,Move,System}, //use 简化路径，这里可以直接使用Command::Edit，Command::Move,Command::System，这几个变体
        System::{Quit,Resize,Save} //use 简化路径，这里可以直接使用 System 的几个变体
    },
    messagebar::Messagebar,
    terminal::Size,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");//版本号
const NAME: &str = env!("CARGO_PKG_NAME");//文件名
const QUIT_TIMES: u8 = 3; //退出确认次数



pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
    message_bar: Messagebar,
    terminal_size: Size, 
    title: String,
    quit_times: u8,
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

        let mut editor = Editor::default();
        let size = Terminal::size().unwrap_or_default();

        //更新terminal_sizey 以及 message_bar 和 status_bar 的位置信息
        editor.resize(size); 

        //更新message.bar的文字信息
        editor.message_bar.update_message("HELP: Ctrl-s = save | Ctrl-c = quit"); 

        //更新view
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            if editor.view.load(&file_name).is_err() {
                editor.message_bar.update_message(&format!("ERR: Could not open file: {file_name}"));
            }
        }

        
        //更新状态栏的文字信息
        editor.refresh_statusbar();
        
        Ok(editor)
    }

    ///更新editor的terminal_size 以及 成员中需要的terminal_size
    fn resize(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize( Size { 
            columns: size.columns,
            rows: size.rows.saturating_sub(2) //预留两行空间
        });

        self.message_bar.resize( Size {
            columns: size.columns,
            rows: 1, 
        });

        self.status_bar.resize( Size { 
            columns: size.columns,
            rows: 1, 
        });
    }

    ///主要运行逻辑
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

        //应该进行处理
        if let Ok(command) = Command::try_from(event) {
            self.process_command(command);
        } 

    } 
    

    ///处理command指令
    fn process_command(&mut self,command: Command) {
        //首先匹配 Quit,Resize
        match command {
            System(Quit) => self.handle_quit(),
            System(Resize(size)) => self.resize(size),
            _ => self.reset_quit_times(),
        }

        match command {
           System(Quit | Resize(_)) => {}, //上面已经处理好了 
           System(Save) => self.handle_save(),
           Edit(edit_command) => self.view.handle_edit_command(edit_command), 
           Move(move_command) => self.view.handle_move_command(move_command),
        }
    }

    ///当文件没有被修改的时候，可以直接退出;当quit_times == 3 的时候可以直接退出;其余，增加quit_times 的次数
    fn handle_quit(&mut self) {
       if !self.view.get_status().is_modified || self.quit_times + 1 == QUIT_TIMES {
            self.should_quit = true;
       } else if self.view.get_status().is_modified {
            self.quit_times += 1;  
       } 
    }

    ///处理保存指令
    fn handle_save(&mut self) {
        if self.view.save().is_ok() {
            self.message_bar.update_message("File saved successfully");
        } else {
            self.message_bar.update_message("Error writing file");
        }
    }


    ///重置键入退出指令次数为0,当quit_times不为0的时候
    fn reset_quit_times(&mut self) {
        if self.quit_times > 0 {
            self.quit_times = 0;
            self.message_bar.update_message("");
        }
    }

    fn refresh_screen(&mut self) {//刷新屏幕，这里忽略了对error的处理，即使发生也只是光标是否可见的问题
        if self.terminal_size.rows == 0 || self.terminal_size.columns == 0 {
            return;
        }

        let _ = Terminal::hide_caret();

        //渲染message_bar
        self.message_bar.render(self.terminal_size.rows.saturating_sub(1)); //在倒数第一行显示

        //渲染message_bar
       if self.terminal_size.rows > 1 {
            self.status_bar.render(self.terminal_size.rows.saturating_sub(2));
       } 

       //渲染View
       if self.terminal_size.rows > 2 {
            self.view.render(0);
       } 

       
        let _ = Terminal::move_caret_to(self.view.caret_position());

        let _ = Terminal::show_caret();
        let _ = Terminal::execute();

    }

    ///更新状态栏关于文件的信息
    pub fn refresh_statusbar(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME}", status.file_name); //注意，NAME是全局的变量参数

        self.status_bar.update_status(status);//更新状态栏的文件参数

        //正确设置终端的文件显示名
        if title != self.title && matches!(Terminal::set_title(&title),Ok(())) {
            self.title = title;
        }
    }

    

}

impl Default for Editor {
    fn default() -> Self {
        Self { 
            should_quit: false,
            view: View::default(),
            status_bar: StatusBar::default(), 
            message_bar: Messagebar::default(), 
            terminal_size: Size::default(), 
            title: String::default(), 
            quit_times: 0,
        }
    }
}