// git feature
// what to do ? (k/d/s/?) > s
//

use std::io;
use std::io::{Read, Write};

fn main() -> Result<(), Error> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    //    stdout.write_all(b"hello world\n").unwrap();
    let mut stdin = io::stdin().bytes();
    loop {
        write!(stdout, "Type something > ")?;
        stdout.flush()?;
        let byte = match stdin.next() {
            Some(byte) => byte?,
            None => break,
        };
        let c = char::from(byte);
        if c == 'q' {
            write!(stdout, "You typed '{}' to quit\n\r", c)?;
            stdout.flush()?;
            break;
        }
        write!(stdout, "You typed '{}'\n\r", c)?;
        stdout.flush()?;
    }
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    CrosstermError(#[from] crossterm::ErrorKind),
    #[error(transparent)]
    IoError(#[from] io::Error),
}
