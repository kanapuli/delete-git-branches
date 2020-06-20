// git feature
// what to do ? (k/d/s/?) > s
//

use std::io;
use std::io::{Read, Write};

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    //    stdout.write_all(b"hello world\n").unwrap();
    let mut stdin = io::stdin().bytes();
    loop {
        write!(stdout, "Type something > ").unwrap();
        stdout.flush().unwrap();
        let c = char::from(stdin.next().unwrap().unwrap());
        if c == 'q' {
            write!(stdout, "You typed '{}' to quit\n\r", c).unwrap();
            stdout.flush().unwrap();
            break;
        }
        write!(stdout, "You typed '{}'\n\r", c).unwrap();
        stdout.flush().unwrap();
    }
    crossterm::terminal::disable_raw_mode().unwrap();
}
