mod cpu;
mod mem;
use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::default();
    cpu.set_reg(0b111, 0x31);
    cpu.execute(0b00010111);
    cpu.execute(0b00010111);
    cpu.print_state();
    println!("---------------------------------------");
    cpu.execute(0b00010111);
    cpu.print_state();
    println!("---------------------------------------");
    cpu.execute(0b00010111);
    cpu.print_state();
}
