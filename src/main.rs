mod cpu;
mod bus;
mod utils;

use std::io;
use std::io::Write;

#[allow(unused_imports)]
use crate::cpu::CPU;
#[allow(unused_imports)]
use crate::bus::Bus;


#[allow(unused_variables, dead_code)]
fn main() {
    let step_by_step: bool = false;
    let mut cpu = CPU::default();
    let mut bus = Bus::default();
    // clear!();
    loop {
        let word = input!("> $ ");
        let cmd = word.as_str().split_whitespace().collect::<Vec<_>>();
        if !cmd.is_empty() {
            match cmd[0] {
                "exit" | "q" | "quit" => break,
                "cls" | "clear" => clear!(),
                _ => eprintln!("Unknown command: {word}"),
            }
        }
    }
}
