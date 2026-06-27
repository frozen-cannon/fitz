use std::{
    env::{self},
    process,
};

fn main() {
    let path = env::current_dir().unwrap_or_else(|err| {
        eprintln!("Problem while getting the current directory: {}", err);
        process::exit(1);
    });
    println!("{path:?}");

    let args: Vec<String> = env::args().collect();

    for arg in args {
        println!("{arg}")
    }

    Command::help();
}

struct Command {
    pub name: String,
    pub description: String,
}

impl Command {
    fn help() -> String {
        format!("Thanks for using fits")
    }
}
