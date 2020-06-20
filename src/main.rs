// git feature
// what to do ? (k/d/s/?) > s
//

use git2::{BranchType, Repository};
use std::io;
use std::io::{Read, Write};

fn main() -> Result<(), Error> {
    crossterm::terminal::enable_raw_mode()?;
    git()?;
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

fn git() -> Result<(), Error> {
    let mut stdout = io::stdout();
    let repo = Repository::open_from_env()?;
    for branch in repo.branches(Some(BranchType::Remote))? {
        let (branch, branch_type) = branch?;
        let name = branch.name_bytes()?;
        stdout.write_all(name)?;
        write!(stdout, "\n\r")?;
        let commit = branch.get().peel_to_commit()?;
        println!("{}", commit.id());
    }
    Ok(())
}
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    CrosstermError(#[from] crossterm::ErrorKind),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    GitError(#[from] git2::Error),
}
