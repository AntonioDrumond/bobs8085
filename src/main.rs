mod assembler;
mod bus;
mod cpu;
mod utils;

use std::io;
use std::io::Write;

use assembler::assemble;
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

fn run_step(cpu: &mut CPU, bus: &mut Bus) {

    let old = bus.mem_clone();
    let mem_diff : Vec<(u16, u8)> = bus.mem_diff(old);
    println!("n = {}", mem_diff.len());

//	    cpu.set_pc(0xC000);
//	    let mut running = true;
//	    while running {
//	        
//	        let in = input!().to_lowercase();
//	        match in {
//	            ">" | "forward" | "f" => (),
//	            "<" | "backward" | "b" => (),
//	        }
//	        running = cpu.execute(bus);
//	    }
//	    println!("\nCPU State at end of program:\n");
//	    cpu.print_state();
//	    println!("\n");
//	    match bus.mem_write_file("./memory.txt") {
//	        Ok(()) => println!("Memory saved to \"./memory.txt\""),
//	        Err(e) => eprintln!("Error printing memory: {e}"),
//	    }
//	    match bus.io_write_file("./io.txt") {
//	        Ok(()) => println!("IO ports saved to \"./io.txt\""),
//	        Err(e) => eprintln!("Error printing IO: {e}"),
//	    }

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
