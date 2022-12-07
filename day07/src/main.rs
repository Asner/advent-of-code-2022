use std::{collections::HashMap, hash::Hash};

struct File {
    name: String,
    size: usize,
}

struct Directory {
    path: String,
    files: Vec<File>,
    directories: Option<HashMap<String, Directory>>,
}

fn main() {
    let input = include_str!("./input.txt");
    part_one(input)
}

fn part_one(input: &str) {
    build_directories_from_input(input);
}

fn build_directories_from_input(input: &str) {
    let root_directory: Directory = Directory {
        path: "/".to_string(),
        files: vec![],
        directories: Some(HashMap::new()),
    };
    let mut current_directory: &Directory = &root_directory;
    for (_i, line) in input.lines().enumerate() {
        if line.starts_with("$") {
            let (command, argument) = process_command(line);
            if command == "cd" {
                let parent_directory = current_directory;
                let new_directory = Directory {
                    path: argument,
                    files: vec![],
                    directories: Some(HashMap::new()),
                };

                current_directory = match argument.as_str() {
                    ".." => parent_directory,
                    "/" => &root_directory,
                    _ => &new_directory,
                }
            };
            println!("\t{:?}", current_directory.path);
        }
    }
}

fn process_command(line: &str) -> (String, String) {
    let split_command: Vec<&str> = line.split(" ").collect();
    let command = split_command[1].to_owned();
    let argument = split_command.get(2).clone();
    return (command, argument.unwrap_or(&&"").to_string());
}

fn process_cd(current_dir: String, new_dir: &str) -> String {
    let mut split_dir = current_dir.split("/").into_iter().collect::<Vec<&str>>();
    match new_dir {
        ".." => {
            split_dir.pop();
        }
        "/" => split_dir = Vec::<&str>::new(),
        _ => {
            split_dir.insert(split_dir.len(), new_dir);
        }
    };
    return split_dir.join("/").to_string();
}
