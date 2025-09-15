#[derive(Copy, Clone)]
pub enum Register {
    A, B, C, D, E, H, L,
}

#[derive(Copy, Clone)]
pub enum Flag {
    Z, S, AC, CY, P,
}

pub enum CPU_E {
    registers: Vec<(Register, u8)>,
    flags: Vec<(Flag, boll)>,
}   

pub enum Changes {
    cpu: CPU,
    memory: Vec<(u16, u8)>,
}
