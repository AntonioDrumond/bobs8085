#[macro_export]
macro_rules! clear {
    () => {
        {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            let _ = io::stdout().flush();
        }
    };
}

#[macro_export]
macro_rules! input {
    ($a:ident) => {
        std::io::stdin().read_line(&mut $a).unwrap();
    };

    () => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line
        }
    };

    ($t:expr) => {
        {
            print!("{}", $t);
            let _ = io::stdout().flush();
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line
        }
    };

    ($t:expr, $a:ident) => {
        print!("{}", $t);
        let _ = io::stdout().flush();
        std::io::stdin().read_line(&mut $a).unwrap();
    };
}
