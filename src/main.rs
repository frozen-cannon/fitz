mod arg_parser;
mod dummy;
use std::{env, process};

fn main() {
    // let path = env::current_dir().unwrap_or_else(|err| {
    //     eprintln!("Problem while getting the current directory: {}", err);
    //     process::exit(1);
    // });
    let mut args: Vec<String> = env::args().collect();

    let mut commands: Vec<Command> = vec![];

    // Command declaration
    let base_command: Command = Command {
        call_name: "dummy".to_string(),
    };
    commands.push(base_command);

    if args.len() < 2 {
        println!("This tool is meant to run any of the following commands:");
        for command in commands {
            println!("{}", command.call_name)
        }
        process::exit(1);
    }
    let command_name = args[1].clone();
    println!("Looking for {command_name} command on the command list");

    args.drain(0..2);
    match command_name {
        val if val == "dummy".to_owned() => dummy::dummy::base(args.clone()),
        _ => {
            terminate(ArgumentError::ToFew);
        }
    }
}

struct Command {
    call_name: String,
}

fn terminate(err: ArgumentError) {
    match err {
        ArgumentError::ToFew => {
            panic!("Error, not enough arguments for this function!")
        }
        ArgumentError::ToMany => {
            panic!("Error, too many arguments for this function!")
        }
    }
}

enum ArgumentError {
    ToMany,
    ToFew,
}

// TODO: Make a parse function for the args that parse args like this:
// if it start with no hyphen `-` then is

struct CommandArgs {
    arg_name: String,
    // is_required: bool,
    // require_params: bool,
    help_message: String,
}
struct CommandParam {
    param_name: String,
    is_required: bool,
}
