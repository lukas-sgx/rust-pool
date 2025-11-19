use std::io::{self, Write};

pub fn my_putchar(c: char) {
    let mut out = io::stdout();
    let mut buf = [0u8; 4];
    let s = c.encode_utf8(&mut buf);

    let _ = out.write_all(s.as_bytes());
    let _ = out.flush();
}