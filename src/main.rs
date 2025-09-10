mod assembler;
mod bus;
mod cpu;

use assembler::assemble;

fn main() {
    match assemble("test/code.asm", "b") {
        Ok(()) => println!("OK!"),
        Err(error) => panic!("HERES Y UR DUMB: {error:?}"),
    }
}
