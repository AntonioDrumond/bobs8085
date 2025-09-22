mod utils;

use std::{
    io,
    io::Write,
};

use bobs8085::{
    changes::Changes,
    Simulator,
    //cpu::CPU,
    //bus::Bus,
    assemble,
};

use utils::{
    clear,
    parse_u16,
};

// fn run_all(cpu: &mut CPU, bus: &mut Bus) {
fn run_all(sim: &mut Simulator) {
    sim.set_pc(0xC000);
    let mut running = true;
    while running {
        running = sim.execute();
    }
    println!("\nCPU State at end of program:\n");
    sim.print_state();
}

// fn run_step(cpu: &mut CPU, bus: &mut Bus) {
fn run_step(sim: &mut Simulator) {
    sim.set_pc(0xC000);

    let mut changes: Vec<Changes> = vec![];
    let mut start = Changes::default();
    start.cpu.pc = 0xC000;
    changes.push(start);

    let mut running = true;
    let mut step = 0;

    while running {
        utils::clear();
        println!("step: {step}\n");
        sim.print_state();

        let line = input!(
            "Options:\n
[F]/[Forward]/[>]  => Go forward 1 step\n
[S]/[Stop]/[Exit]/[|]  => Exit step by step execution\n
[B]/[Backward]/[<]  => Go back 1 step\n
[P]/[Print]/[Print + range]  => Print the memory\n
> $ "
        )
        .to_lowercase();
        let cmd = line.as_str().split_whitespace().collect::<Vec<_>>();

        if !cmd.is_empty() {
            match cmd[0] {
                ">" | "forward" | "f" => {
                    if cmd.len() >= 2 {
                        let n = cmd[1].parse().expect("Not a valid number");
                        let mut i = 0;
                        while i < n && running == true {
                            let (cpu_old, mem_old, io_old) = sim.clone_cpu_bus();

                            running = sim.execute();

                            let diff = sim.get_changes(cpu_old, mem_old, io_old);
                            changes.push(diff);

                            step += 1;
                            i += 1;
                        }
                    } else {
                        let (cpu_old, mem_old, io_old) = sim.clone_cpu_bus();
                        running = sim.execute();
                        let diff = sim.get_changes(cpu_old, mem_old, io_old);
                        changes.push(diff);
                        step += 1;
                    }
                }
                "<" | "backward" | "b" => {
                    if step != 0 {
                        step -= 1;
                        sim.restore(&changes[step]);
                    } else {
                        println!("Already at the start!");
                    }
                }
                "|" | "stop" | "s" | "exit" => {
                    clear();
                    running = false;
                }
                "p" | "print" => {
                    let len = cmd.len();
                    if len == 1 {
                        sim.print_program();
                    } else if len == 2 {
                        let mut val = 0x00;
                        match parse_u16(cmd[1]) {
                            Ok(res) => val = res,
                            Err(err) => eprintln!("ParseError: {}", err),
                        }
                        sim.print_mem_range(val, val+1);
                    } else if len == 3 {
                        let mut lo = 0x00;
                        let mut hi = 0x00;
                        match parse_u16(cmd[1]) {
                            Ok(res) => lo = res,
                            Err(err) => eprintln!("ParseError: {}", err),
                        }

                        match parse_u16(cmd[2]) {
                            Ok(res) => hi = res,
                            Err(err) => eprintln!("ParseError: {}", err),
                        }
                        if lo > hi {
                            let tmp = lo;
                            lo = hi;
                            hi = tmp;
                        }
                        sim.print_mem_range(lo, hi);
                    }
                    let _ = input!("\nPress [Enter] to continue\n");
                }
                _ => println!("\"{}\" is not recognized as a command", cmd[0]),
            }
        }
    }

    clear();
    println!("Program finished.\nCPU State at end of program:\n");
    sim.print_state();
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
                "assemble" => {
                    if cmd.len() < 3 { eprintln!("Please provide a input file and an output file for command \"assemble\""); }
                    else {
                        match assemble(cmd[1], cmd[2]) {
                            Ok(()) => println!("Binary file saved at \"bin/{}.bin\"", cmd[2]),
                            Err(err) => panic!("{}", err),
                        }
                    }
                }
                "run" => {
                    if cmd.len() < 2 { eprintln!("Please provide a file name for command \"run\""); }
                    else {
                        match cmd[1] {
                            "step" => {
                                if cmd.len() < 3 { eprintln!("Please provide a file name for command \"run step\""); }
                                else {
                                    let fname = cmd[2]
                                        .split("/").collect::<Vec<_>>().last().expect("REASON")
                                        .split(".").collect::<Vec<_>>()[0];

                                    let outfile = format!("bin/{fname}.bin");
                                    match assemble(cmd[2], fname) {
                                        Ok(_) =>   run_step(&mut Simulator::bus_from_file(&outfile)),
                                        Err(err) => panic!("{}", err),
                                    }
                                }
                            }
                            "bin" => {
                                if cmd.len() < 3 { eprintln!("Please provide a file name for command \"run bin\""); }
                                else {
                                    match cmd[2] {
                                        "step" => {
                                            if cmd.len() < 4 { eprintln!("Please provide a file name for command \"run bin step\""); }
                                            else {
                                                run_step(&mut Simulator::bus_from_file(cmd[3]));
                                            }
                                        }
                                        _ => run_all(&mut Simulator::bus_from_file(cmd[2])),
                                    }
                                }
                            },
                            _ => {
                                let fname = cmd[1]
                                    .split("/").collect::<Vec<_>>().last().expect("REASON")
                                    .split(".").collect::<Vec<_>>()[0];

                                let outfile = format!("bin/{fname}.bin");
                                match assemble(cmd[1], fname) {
                                    Ok(_) =>   run_all(&mut Simulator::bus_from_file(&outfile)),
                                    Err(err) => panic!("{}", err),
                                }
                            }
                        }
                    }
                }
                _ => eprintln!("Unknown command: {word}\nYou can type \"help\" to see available commands or \"quit\" to exit."),
            }
        }
    }
}
