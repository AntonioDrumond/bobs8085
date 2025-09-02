#[derive(Debug)]
#[allow(dead_code, unused_variables)]
pub struct CPU {
    a: u8, // Accumulator
    b: u8, // Pair BC
    c: u8,
    d: u8, // Pair DE
    e: u8,
    h: u8, // Pair HL
    l: u8,
    sp: u16, // Stack Pointer
    pc: u16, // Program Counter
    // Flags:
    s: bool,  // Sign
    z: bool,  // Zero
    ac: bool, // Auxiliary Carry
    p: bool,  // Parity
    cy: bool, // Carry
}

#[allow(dead_code, unused_variables)]
impl CPU {}
