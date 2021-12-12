#![feature(io_read_to_string)]
use git2::{BranchType, Repository};
use std::{env, io};
fn main() -> io::Result<()> {
    let path = env::current_dir()?;
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("Error while opening the repository path: {}", e),
    };

    let head_ref = repo.head().unwrap();

    let head = match head_ref.peel_to_commit() {
        Ok(head) => head,
        Err(e) => panic!("Error while reading the head commit: {}", e),
    };

    let branches = repo.branches(Some(BranchType::Local));

    for branch in branches.unwrap() {
        let (mut b, t) = branch.unwrap();
        if t == BranchType::Local {
            match b.get().name() {
                Some(b) => {
                    if b == "refs/heads/master" {
                        continue;
                    } else {
                        println!("Checking {:?}", b)
                    }
                }
                None => (),
            }

            let target = match b.get().peel_to_commit() {
                Ok(target) => target,
                Err(e) => panic!("Error while getting the target commit: {}", e),
            };
            let merged = repo.merge_base(head.id(), target.id()).unwrap();
            println!("=>    {:?} - {}", head.id(), merged);
            if merged == head.id() {
                println!("deleteing branch {:?}", b.get().name());
                let _ = b.delete().unwrap();
            }
        }
    }

    Ok(())
}
