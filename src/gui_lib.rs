use bobs8085::{
    changes::Changes,
    Simulator,
};

use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use iced::widget::text_editor;

use directories::ProjectDirs;


#[derive(Debug, Clone)]
pub enum Message {
    SetInterface(u8), // 0 -> Simulator 
                      // 1 -> Open file
                      // 2 -> Help

    OpenFile(PathBuf),
    SelectFile(PathBuf),
    NavigateTo(PathBuf),
    SaveFile,

    Assemble,
    RunAll,
    RunStep,

    EditText(text_editor::Action),

    MemoryPage(u8),

    HelpPage(u8), // 0 -> Arithmetic
                  // 1 -> Branching
                  // 2 -> Control
                  // 3 -> Data Transfer
                  // 4 -> Logical

    ForwardStep,
    BackwardStep,
    StopStep,
}

#[derive(Debug)]
pub struct State {

    pub interface: u8, // 0 -> simulator | 1 -> open file | 3 -> help page
    pub current_memory_page: u8,
    pub current_help_page: u8,

    pub cwd: PathBuf,
    pub selected_file: PathBuf,
    pub current_file: PathBuf,
    pub simulator_path: PathBuf,

    pub editor_content: text_editor::Content,
    pub assemble_error: bool,
    pub logging_message: String,

    pub sim: Simulator,
    pub step: bool,
    pub changes: Vec<Changes>,
}

impl Default for State {
    fn default() -> Self {
        let mut sim_path = PathBuf::default();
        let mut cwd = PathBuf::default();
        match env::current_dir() {
            Ok(path) => {
                cwd = path.clone();
                sim_path = path;
            }
            Err(err) => eprintln!("{}", err),
        };
        if let Some(dir) = ProjectDirs::from("org", "Simulator", "bobs8085") {
            let config_dir = dir.config_dir().to_path_buf();
            match config_dir.try_exists() {
                Ok(status) => {
                    let mut last_dir = config_dir.clone();
                    last_dir.push("last_dir.txt");

                    let mut simulator_path = config_dir.clone();
                    simulator_path.push("simulator_path.txt");

                    if !status {
                        match fs::create_dir_all(config_dir.clone()) {
                            Err(err) => eprintln!("{}", err),
                            Ok(_) => {

                                // Store the directory of the most recent opened file
                                match File::create(last_dir) { 
                                    Ok(mut file) => {
                                        let _ = write![file, "{}", cwd.to_str().unwrap()];
                                    },
                                    Err(err) => eprintln!("{}", err),
                                }; 

                                // Store the directory of simulator             !!! There could be a problem if someone opens the simulator for the first time
                                                                                // outside of the simulator folder (or does not have the config dir). 
                                                                                // Hopefuly that does not happen :)
                                match File::create(simulator_path) { 
                                    Ok(mut file) => {
                                        let _ = write![file, "{}", cwd.to_str().unwrap()];
                                    },
                                    Err(err) => eprintln!("{}", err),
                                };

                            },
                        };
                    } else {
                        match fs::read_to_string(last_dir) {
                            Ok(res) => {
                                cwd = PathBuf::from(res); 
                            },
                            Err(err) => eprintln!("\n{}", err),
                        };
                        match fs::read_to_string(simulator_path) {
                            Ok(res) => {
                                sim_path = PathBuf::from(res); 
                            },
                            Err(err) => eprintln!("\n{}", err),
                        };
                    }
                }
                Err(err) => eprintln!("{}", err),
            };
        }
        let mut state = State {
            sim: Simulator::default(),
            editor_content: text_editor::Content::default(),
            assemble_error: false,
            logging_message: String::new(),

            interface: 0,

            cwd: cwd,
            simulator_path: sim_path,
            selected_file: PathBuf::default(),
            current_file: PathBuf::default(),

            current_memory_page: 0,
            current_help_page: 0,
            
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

    pub fn update_last_dir(&mut self) {
        if self.cwd.is_dir() {
            if let Some(dir) = ProjectDirs::from("org", "bobs8085", "Simulator") {
                let config_dir = dir.config_dir().to_path_buf();
                match config_dir.try_exists() {
                    Ok(status) => {
                        if status {
                            let mut last_dir = config_dir.clone();
                            last_dir.push("last_dir.txt");
                            match File::create(last_dir.clone()) { 
                                Ok(mut file) => {
                                    let _ = write![file, "{}", self.cwd.to_str().unwrap()];
                                },
                                Err(err) => eprintln!("{}", err),
                            }
                        } else { 
                            eprintln!("Could not update the last directory!");
                        }
                    },
                    Err(err) => eprintln!("Could not update the last directory!\n{}", err),
                };
            }
        }
    }
}
