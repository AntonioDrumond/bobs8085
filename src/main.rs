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

#[allow(unused_variables, dead_code)]
fn run_step(cpu: &mut CPU, bus: &mut Bus) {
    todo!("Step by step execution");
}

fn main() {
    // utils::clear();
    {
//          let mut mem = Mem::default();
        let mut bus = Bus::default();
        bus.mem_set16(0xC000, 0x3E01);
        bus.mem_set16(0xC002, 0x3250);
        bus.mem_set16(0xC004, 0xC032);
        bus.mem_set16(0xC006, 0x51C0);
        bus.mem_set16(0xC008, 0x3E00);
        bus.mem_set16(0xC00A, 0x0E09);
        bus.mem_set16(0xC00C, 0x2150);
        bus.mem_set16(0xC00E, 0xC07E);
        bus.mem_set16(0xC010, 0x2346);
        bus.mem_set16(0xC012, 0x2380);
        bus.mem_set16(0xC014, 0x2777);
        bus.mem_set16(0xC016, 0x2B0D);
        bus.mem_set16(0xC018, 0xC20F);
        bus.mem_set16(0xC01A, 0xC076);
        bus.mem_dump("test.bin");
        bus.mem_write_file("out");
    }
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
