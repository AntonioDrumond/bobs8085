mod cpu;
mod bus;
use crate::cpu::CPU;
use crate::bus::Bus;

fn main() {
    let mut bus = Bus::default();
    let mut cpu = CPU::default();

    cpu.set_pc(0xc000);
    bus.mem_set16(0xc000, 0xcd04);
    bus.mem_set16(0xc002, 0xc076);
    bus.mem_set16(0xc004, 0xcd08);
    bus.mem_set16(0xc006, 0xc0c9);
    bus.mem_set16(0xc008, 0xc900);

    cpu.print_state();
    bus.mem_print();
    while true {
        cpu.execute(bus.mem_get8(cpu.get_pc()), &mut bus);
        cpu.print_state();
        bus.mem_print();
    }
    print!("Hello World!");
}
