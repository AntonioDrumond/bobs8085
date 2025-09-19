use std::io;
use std::io::Write;

#[allow(dead_code)]
pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let _ = io::stdout().flush();
}

#[allow(dead_code)]
pub fn parse_u16(s: &str) -> Result<u16, std::num::ParseIntError> {
    if let Some(hex_str) = s.strip_prefix("0x") {
        u16::from_str_radix(hex_str, 16)
    } else {
        s.parse::<u16>()
    }
}

#[macro_export]
macro_rules! input {
    ($a:ident) => {
        std::io::stdin().read_line(&mut $a).unwrap();
    };

    () => {{
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line
    }};

    ($t:expr) => {{
        print!("{}", $t);
        let _ = io::stdout().flush();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line
    }};

    ($t:expr, $a:ident) => {
        print!("{}", $t);
        let _ = io::stdout().flush();
        std::io::stdin().read_line(&mut $a).unwrap();
    };
}

#[allow(dead_code)]
#[rustfmt::skip]
pub fn help_simulator() {
    println!();
    println!("Simulator commands:");
    println!("h | help                  --> Prints this");
    println!("clear | cls               --> Clear terminal screen");
    println!("exit | quit | q           --> Exit simulator");
    println!("run [FILENAME]            --> Assemble and run (Without step) program in file");
    println!("run step [FILENAME]       --> Assemble and run (Step by step) program in file");
    println!("run bin [FILENAME]        --> Run program from binary memory file");
    println!("run bin step [FILENAME]   --> Run program (Step by step) from binary memory file");
    println!("                          (Program in binary memory file should be between positions");
    println!("                           C000 and CFFF in memory)");
    println!("assemble [INPUT] [OUTPUT] --> Assemble program in plain text file([INPUT]) and creates");
    println!("                               memory file ([OUTPUT])");
    println!();
}
