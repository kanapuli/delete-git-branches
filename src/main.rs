// git feature
// what to do ? (k/d/s/?) > s
//

use chrono::prelude::*;
use chrono::Duration;
use git2::{BranchType, Oid, Repository};
use std::io;
use std::io::{Read, Write};

type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<(), Error> {
    crossterm::terminal::enable_raw_mode()?;
    let repo = Repository::open_from_env()?;
    get_branches(repo)?;
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

struct Branch {
    id: Oid,
    time: NaiveDateTime,
    name: String,
}

fn get_branches(repo: Repository) -> Result<Vec<Branch>> {
    let mut branches = repo
        .branches(Some(BranchType::Remote))?
        .map(|branch| {
            let (branch, _) = branch?;

            let name = String::from_utf8(branch.name_bytes()?.to_vec())?;

            let commit = branch.get().peel_to_commit()?;

            let time = commit.time();
            let offset_time = Duration::minutes(i64::from(time.offset_minutes()));
            let time = NaiveDateTime::from_timestamp(time.seconds(), 0) + offset_time;

            Ok(Branch {
                id: commit.id(),
                time,
                name,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    branches.sort_by_key(|branch| branch.time);
    Ok(branches)
}
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    CrosstermError(#[from] crossterm::ErrorKind),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    GitError(#[from] git2::Error),

    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}
