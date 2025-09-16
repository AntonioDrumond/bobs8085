mod assembler;
mod bus;
mod cpu;
mod utils;

use assembler::assemble;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    assemble("test/bubble.asm", "bubble")?;
    Ok(())
}
