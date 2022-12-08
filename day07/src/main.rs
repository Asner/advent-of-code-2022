use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    let directories: HashMap<String, u32> = build_directories_from_input(input);

    let result = directories
        .into_iter()
        .filter(|entry| entry.1.lt(&100000))
        .map(|entry| entry.1)
        .sum::<u32>();

    println!("Part 1 result: {:?}", result);
}

fn part_two(input: &str) {
    const TOTAL_SPACE_DISK_SIZE: u32 = 70_000_000;
    const SPACE_SIZE_TO_FREE: u32 = 30_000_000;
    let directories: HashMap<String, u32> = build_directories_from_input(input);
    let root_dir_size = directories.get("").unwrap().to_owned();
    let target_to_free = SPACE_SIZE_TO_FREE - (TOTAL_SPACE_DISK_SIZE - root_dir_size);
    let mut result = directories
        .iter()
        .filter(|entry| entry.1.gt(&target_to_free))
        .map(|entry| entry.1.to_owned())
        .collect::<Vec<u32>>();

    result.sort_by(|a, b| a.partial_cmp(&b).unwrap().clone());
    println!("Part 2 result: {}", result.first().unwrap());
}

fn build_directories_from_input(input: &str) -> HashMap<String, u32> {
    let mut directory_map = HashMap::<String, u32>::new();
    let mut current_directory = "".to_owned();
    for (_i, line) in input.lines().enumerate() {
        if line.starts_with("$") {
            let (command, argument) = process_command(line);
            if command == "cd" {
                let mut split_dir = current_directory.split("/").into_iter().collect::<Vec<&str>>();
                match argument.as_str() {
                    ".." => {
                        split_dir.pop();
                    }
                    "/" => split_dir = Vec::<&str>::new(),
                    _ => {
                        split_dir.insert(split_dir.len(), &argument);
                    }
                };
                current_directory = split_dir.join("/").to_string();
                directory_map.entry(current_directory.clone()).or_insert_with(||0);
            };
        } else {
            if line.starts_with("dir") {
                continue;
            }
            let map_line = line.split(" ").collect::<Vec<&str>>();
            let mut dir_build: String= "".to_owned();
            for (_i, dir) in current_directory.split("/").enumerate() {
                dir_build.push_str(dir);
                directory_map.entry(dir_build.clone())
                .and_modify(|entry| {
                    *entry += map_line[0].parse::<u32>().unwrap()
                }).or_insert(0);
                dir_build.push_str("/");
            }
        }
    }
    directory_map
}

fn process_command(line: &str) -> (String, String) {
    let split_command: Vec<&str> = line.split(" ").collect();
    let command = split_command[1].to_owned();
    let argument = split_command.get(2).clone();
    return (command, argument.unwrap_or(&&"").to_string());
}

