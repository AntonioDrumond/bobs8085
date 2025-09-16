mod assembler;
mod bus;
mod cpu;
mod utils;
mod changes;

use std::io::Write;
use std::io;

use crate::cpu::CPU;
use crate::changes::Changes;
use crate::bus::Bus;
use crate::utils::clear;

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

    cpu.set_pc(0xC000);

    let mut changes : Vec<Changes> = vec![];
    let mut start = Changes::default();
    start.cpu.pc = 0xC000;
    changes.push(start);

    let mut running = true;
    let mut step = 0;

    while running {
        
        clear();
        println!("step: {step}\n");
        cpu.print_state();
        println!();
        match bus.mem_write_file("./memory.txt") {
            Ok(()) => println!("Memory saved to \"./memory.txt\""),
            Err(e) => eprintln!("Error printing memory: {e}"),
        }
        match bus.io_write_file("./io.txt") {
            Ok(()) => println!("IO ports saved to \"./io.txt\""),
            Err(e) => eprintln!("Error printing IO: {e}"),
        }

        let line = input!("Options:\n
[B]/[Backward]/[<] => Go back 1 step\n
[Stop]/[Exit]/[|]  => Exit step by step execution\n
[F]/[Forward]/[>]  => Go forward 1 step\n
> $ "
        ).to_lowercase();
        let cmd = line.as_str().split_whitespace().collect::<Vec<_>>();

        let cpu_old = cpu.clone();
        let mem_old = bus.mem_clone();
        // let mut halt = false;

        if !cmd.is_empty() {
            match cmd[0] {
                ">" | "forward" | "f" => {
                    if !cpu.execute(bus) { running = false; } 
                    let diff = { Changes { memory: bus.mem_diff(mem_old), cpu: cpu.diff(cpu_old)} };
                    changes.push(diff);
                    step += 1;
                },
                "<" | "backward" | "b" => {
                    if step != 0 {
                        step -= 1;
                        cpu.restore(bus, &changes[step]);
                    }
                    else { println!("Already at the start!"); }

                },
                "|" | "stop" | "s" | "exit" => { 
                    clear();
                    running = false;
                },
                _ => println!("\"{}\" is not recognized as a command", cmd[0]),
            }
        }

        /*
        while halt {
            let line = input!("Program finished! Do you want to exit it?\n\n[y/n]\n\n").to_lowercase();
            let cmd = line.as_str().split_whitespace().collect::<Vec<_>>();

            if !cmd.is_empty() {
                match cmd[0] {
                    "yes" | "y" => running = false,
                    "no" | "n" => halt = false,
                    _ => (),
                }
            }
        }
        */
    }

    clear();
    println!("Program finished.\nCPU State at end of program:\n");
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
                                                run_step(&mut CPU::default(), &mut Bus::from_file(cmd[3]));
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
