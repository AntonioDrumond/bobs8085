use crate::gui_lib::{State, Message};

use bobs8085::{
    Simulator,
    assemble,
};

use std::{
    fs::{
        self,
        File,
    }, io::Write, path::PathBuf
};

use iced::widget::{
    text_editor,
};

fn write_default_file (state: &mut State) {
    match File::create("program.asm") {
        Ok(mut file) => {
            let text = state.editor_content.text();
            let _ = write![file, "{}", text];
        },
        Err(err) => eprintln!("{}", err),
    };
    state.current_file = PathBuf::from("program.asm");
}

pub fn update(state: &mut State, message: Message) {

    match message {
        Message::RunAll => {
            state.reset_changes();
            state.sim.clear_cpu();
            state.sim.set_pc(0xC000);
            while state.sim.execute() {}
        }
        Message::RunStep => {
            state.reset_changes();
            state.sim.clear_cpu();
            state.sim.set_pc(0xC000);
            state.step = true;
        }
        Message::StopStep => {
            state.step = false;
        }
        Message::ForwardStep => {
            let (cpu_old, mem_old, io_old) = state.sim.clone_cpu_bus();
            let status = state.sim.execute();
            if !status {
                state.step = false;
            } else {
                let diff = state.sim.get_changes(cpu_old, mem_old, io_old);
                state.changes.push(diff);
            }
        }
        Message::BackwardStep => {
            if state.changes.len() > 1 {
                match &state.changes.pop() {
                    Some(changes) => state.sim.restore(changes),
                    _ => (),
                }
            }
        }
        Message::MemoryPage(page) => state.current_memory_page = page,
        Message::EditText(action) => state.editor_content.perform(action),
        Message::Assemble => {
            state.step = false;
            if !state.current_file.exists() {
                write_default_file(state);
            } else {
                match File::create(state.current_file.clone()) {
                    Ok(mut file) => {
                        let _ = write![file, "{}", state.editor_content.text()];
                    },
                    Err(err) => eprintln!("{}", err),
                }
            }
            let file_path = state.current_file.to_str().unwrap();
            let file_name = state.current_file.file_stem().unwrap().to_str().unwrap();

            let _ = match assemble(file_path, file_name) {
                Ok(()) => {
                    state.assemble_error = false;
                    state.sim = Simulator::bus_from_file(&format!("bin/{}.bin", file_name));
                    state.reset_changes();
                }
                Err(err) => {
                    state.assemble_error = true;
                    state.logging_message = format!("{}", err);
                }
            };
        },
        Message::SetInterface(interface) => state.interface = interface,
        Message::NavigateTo(path) => {
            state.cwd = path;
            state.selected_file = PathBuf::default();
        }
        Message::SelectFile(file) => state.selected_file = file,
        Message::OpenFile(file_path) => {
            if file_path.exists() {
                match &fs::read_to_string(file_path.clone()) {
                    Ok(res) => {
                        state.editor_content = text_editor::Content::with_text(res);
                        state.current_file = file_path;
                        state.selected_file = PathBuf::default();
                        state.interface = 0x0;
                        state.update_last_dir();
                    },
                    Err(err) => eprintln!("{}", err),
                }
            }
        },
        Message::SaveFile => {
            if !state.current_file.exists() {
                write_default_file(state);
            }
            match File::create(state.current_file.clone()) {
                Ok(mut file) => {
                    let text = state.editor_content.text();
                    let _ = write![file, "{}", text];
                },
                Err(err) => eprint!("{}", err),
            }
        },
        Message::HelpPage(page) => state.current_help_page = page,
    }
}
