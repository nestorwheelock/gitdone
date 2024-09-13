use std::env;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    let commit_message = if args.len() > 1 {
        args[1].clone()
    } else {
        // If no argument is provided, ask for a commit message
        print!("Enter a commit message: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    };

    // Run the git commands
    Command::new("git")
        .arg("add")
        .arg(".")
        .status()
        .expect("failed to add files");

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .status()
        .expect("failed to commit");

    Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main")
        .status()
        .expect("failed to push");
}

