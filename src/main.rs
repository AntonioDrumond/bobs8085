mod cpu;
use crate::cpu::CPU;

fn main() {
    CPU::execute(0b01110110);
    print!("Hello World");
}
