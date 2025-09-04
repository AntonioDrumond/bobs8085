mod cpu;
mod mem;
use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::default();
    cpu.execute(0b01110110);
    print!("Hello World");
}
