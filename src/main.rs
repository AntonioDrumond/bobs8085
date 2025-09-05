mod cpu;
mod bus;
use crate::cpu::CPU;
use crate::bus::Bus;

fn main() {
    let mut bus = Bus::default();
    let mut cpu = CPU::default();
    cpu.pc = 0xC003;
    println!("------------- BEFORE ---------------");
    cpu.print_state();
    bus.mem_print();
    cpu.execute(0xCD, &mut bus);
    println!("------------- AFTER ---------------");
    cpu.print_state();
    bus.mem_print();
}
