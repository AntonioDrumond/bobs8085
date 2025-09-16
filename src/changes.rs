#[derive(Default, Debug, Clone, Copy)]
pub struct Regs {

    pub a : u8,
    pub b : u8,
    pub c : u8,
    pub d : u8,
    pub e : u8,
    pub h : u8,
    pub l : u8,
    pub pc : u16,
    pub sp : u16,  

    pub z : bool,
    pub s : bool,
    pub ac : bool,
    pub cy : bool,
    pub p : bool,

}

#[derive(Default, Debug, Clone)]
pub struct Changes {
    pub cpu : Regs,
    pub memory : Vec<(u16, u8)>,
}
