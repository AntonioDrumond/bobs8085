use bobs8085::{
    changes::Changes,
    Simulator,
};

use std::{
    env,
    path::PathBuf,
};

use iced::widget::text_editor;


#[derive(Debug, Clone)]
pub enum Message {
    SetInterface(u8), // 0 -> Simulator 
                      // 1 -> Open file
                      // 2 -> Save file
                      // 3 -> Help

    OpenFile(PathBuf),
    SelectFile(PathBuf),
    NavigateTo(PathBuf),
    SaveFile,

    Assemble,
    RunAll,
    RunStep,

    EditText(text_editor::Action),

    MemoryPage(u8),

    ForwardStep,
    BackwardStep,
    StopStep,
}

#[derive(Debug)]
pub struct State {

    pub interface: u8, // 0 -> simulator | 1 -> open file | 2 -> save file

    pub cwd: PathBuf,
    pub selected_file: PathBuf,
    pub current_file: PathBuf,

    pub editor_content: text_editor::Content,
    pub assemble_error: bool,
    pub logging_message: String,

    pub sim: Simulator,
    pub current_memory_page: u8,
    pub step: bool,
    pub changes: Vec<Changes>,
}

impl Default for State {
    fn default() -> Self {
        let mut state = State {
            sim: Simulator::default(),
            editor_content: text_editor::Content::default(),
            assemble_error: false,
            logging_message: String::new(),

            interface: 0,

            cwd: env::current_dir().unwrap(),
            selected_file: PathBuf::default(),
            current_file: PathBuf::default(),

            current_memory_page: 0,
            step: false,
            changes: vec![Changes::default(); 1],
        };
        state.changes[0].cpu.pc = 0xC000;
        state
    }
}

impl State {
    pub fn reset_changes(&mut self) {
        self.changes = vec![Changes::default(); 1];
        self.changes[0].cpu.pc = 0xC000;
    }
}
