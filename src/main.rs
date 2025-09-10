mod assembler;
mod bus;
mod cpu;

use assembler::assemble;

fn main() {
    println!("Hello, World!");
    match assemble("a", "b") {
        Ok(()) => println!("OK!"),
        Err(error) => panic!("HERES Y UR DUMB: {error:?}"),
    }
}
