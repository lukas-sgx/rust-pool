use std::io::{self, Write};

pub fn my_putstr(input: &str) {
    let mut out = io::stdout();
    
    out.write_all(input.as_bytes()).unwrap();
    out.flush().unwrap()
}