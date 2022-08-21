use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub description: Option<String>,
}

impl Todo {
    pub fn new(title: String, description: Option<String>) -> Todo {
        Todo { title, description }
    }
}

pub enum Command {
    Add(Todo),
    Remove(usize),
    List,
    Load(String),
    Save(String),
    Help,
}

fn get_args(input: String) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut is_quoted = false;

    for c in input.chars() {
        match c {
            ' ' if !is_quoted && current.len() > 0 => {
                args.push(current);
                current = String::new();
            }
            ' ' if !is_quoted && current.len() == 0 => continue,
            '"' if !is_quoted => is_quoted = true,
            '"' if is_quoted => {
                args.push(current);
                current = String::new();
                is_quoted = false;
            }
            _ => current.push(c),
        }
    }

    if current.len() > 0 {
        args.push(current);
    }

    return args;
}

pub fn parse_command(input: String) -> Option<Command> {
    let args = get_args(input);

    // NOTE: Match is more convenient when using &[&str]
    match &args.iter().map(|s| s.as_str()).collect::<Vec<&str>>()[..] {
        ["add", title] => Some(Command::Add(Todo::new(title.to_string(), None))),
        ["add", title, desc] => Some(Command::Add(Todo::new(
            title.to_string(),
            Some(desc.to_string()),
        ))),
        ["remove", id] => match id.parse::<usize>() {
            Ok(id) => Some(Command::Remove(id)),
            Err(_) => None,
        },
        ["list"] => Some(Command::List),
        ["load", file] => Some(Command::Load(file.to_string())),
        ["save", file] => Some(Command::Save(file.to_string())),
        ["help"] => Some(Command::Help),
        _ => None,
    }
}
