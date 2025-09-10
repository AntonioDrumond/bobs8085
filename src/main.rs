mod cpu;
mod bus;
mod utils;

use std::io;
use std::io::Write;

use crate::cpu::CPU;
use crate::bus::Bus;

fn run_all(cpu: &mut CPU, bus: &mut Bus) {
    cpu.set_pc(0xC000);
    let mut running = true;
    while running {
        running = cpu.execute(bus);
    }
    println!("\nCPU State at end of program:\n");
    cpu.print_state();
    println!("\n");
    match bus.mem_write_file("./memory.txt") {
        Ok(()) => println!("Memory saved to \"./memory.txt\""),
        Err(e) => eprintln!("Error printing memory: {e}"),
    }
    match bus.io_write_file("./io.txt") {
        Ok(()) => println!("IO ports saved to \"./io.txt\""),
        Err(e) => eprintln!("Error printing IO: {e}"),
    }
}

#[allow(unused_variables, dead_code)]
fn run_step(cpu: &mut CPU, bus: &mut Bus) {
    todo!("Step by step execution");
}

fn main() {
    // utils::clear();
    loop {
        let word = input!("> $ ");
        let cmd = word.as_str().split_whitespace().collect::<Vec<_>>();
        if !cmd.is_empty() {
            match cmd[0] {
                "exit" | "q" | "quit" => break,
                "cls" | "clear" => utils::clear(),
                "h" | "help" => utils::help_simulator(),
                "run" => {
                    if cmd.len() < 2 { eprintln!("Please provide file name for command \"run\""); }
                    else {
                        match cmd[1] {
                            "step" => {
                                if cmd.len() < 3 { eprintln!("Please provide file name for command \"run step\""); }
                                else {
                                    run_step(&mut CPU::default(), &mut Bus::from_file(cmd[2]));
                                }
                            }
                            "bin" => {
                                if cmd.len() < 3 { eprintln!("Please provide file name for command \"run bin\""); }
                                else {
                                    match cmd[2] {
                                        "step" => {
                                            if cmd.len() < 4 { eprintln!("Please provide file name for command \"run bin step\""); }
                                            else {
                                                run_all(&mut CPU::default(), &mut Bus::from_file(cmd[3]));
                                            }
                                        }
                                        _ => run_all(&mut CPU::default(), &mut Bus::from_file(cmd[2])),
                                    }
                                }
                            },
                            _ => run_all(&mut CPU::default(), &mut Bus::from_file(cmd[1])),
                        }
                    }
                }
                _ => eprintln!("Unknown command: {word}\nYou can type \"help\" to see available commands or \"quit\" to exit."),
            }
        }
    }
}
