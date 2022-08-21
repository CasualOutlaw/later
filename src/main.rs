use std::fs;

use rustyline::{error::ReadlineError, Editor};
use todo::*;

mod todo;

const HELP_MESSAGE: &str = include_str!("help.txt");

fn main() {
    let mut rl = Editor::<()>::new().unwrap();

    let mut items: Vec<Todo> = Vec::new();

    println!("Welcome to LATER, your favorite procrastination tool. Use the `help` command to view a list of commands.");

    loop {
        let input = rl.readline("later> ");

        match input {
            Ok(line) => match parse_command(line) {
                Some(command) => match command {
                    Command::Add(item) => {
                        let title = item.title.to_owned();
                        items.push(item);
                        println!("Added '{}'", title);
                    }
                    // NOTE: `index` is user input, and it starts at 1 unlike Rust where indexes start at 0. So we have to subtract 1.
                    Command::Remove(index) => match items.get(index - 1) {
                        Some(item) => {
                            let title = item.title.to_owned();
                            items.remove(index - 1);
                            println!("Removed '{}'", title);
                        }
                        None => {
                            println!(
                                "Current list has {} items, cannot remove {}-th item",
                                items.len(),
                                index
                            );
                        }
                    },
                    Command::List => {
                        println!("There are {} items in your list.\n", items.len());
                        for item in &items[..] {
                            println!("LATER: {}", item.title);
                            if let Some(desc) = &item.description {
                                println!("- {}", desc);
                            }
                        }
                    }
                    Command::Load(file) => match fs::read_to_string(&file) {
                        Ok(contents) => match serde_json::from_str(contents.as_str()) {
                            Ok(obj) => items = obj,
                            Err(_) => println!("Error parsing JSON from '{}'", &file),
                        },
                        Err(_) => println!("Error while reading from '{}'", &file),
                    },
                    Command::Save(file) => {
                        if let Err(_) =
                            fs::write(&file, serde_json::to_string_pretty(&items).unwrap())
                        {
                            println!("Error while writing to '{}'", &file);
                        }
                    }
                    Command::Help => println!("{}", HELP_MESSAGE),
                },
                None => {
                    println!("Error parsing command. Maybe the command doesn't exist or the usage is wrong?");
                }
            },
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D. Exiting...");
                break;
            }
            Err(_) => {
                println!("Unexpected error while reading input. Exiting...");
                break;
            }
        }
    }
}
