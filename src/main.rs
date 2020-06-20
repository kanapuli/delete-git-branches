// git feature
// what to do ? (k/d/s/?) > s
//

use std::io;
use std::io::Write;

fn main() {
    let mut stdout = io::stdout();
    stdout.write_all(b"hello world\n").unwrap();
}
