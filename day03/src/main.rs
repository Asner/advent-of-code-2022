use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    let result: _ = input
        .lines()
        .map(|rucksack| rucksack_common_char(rucksack))
        .sum::<usize>();

    println!("{:?}", result)
}

fn part_two(input: &str) {
    let vectors = slice_input(input);
    let result: _ = vectors
        .into_iter()
        .map(|groups| {
            let first_group: HashSet<char> = groups.get(0).unwrap().chars().collect();
            let second_group: HashSet<char> = groups.get(1).unwrap().chars().collect();
            let third_group: HashSet<char> = groups.get(2).unwrap().chars().to_owned().collect();
            let intersection: HashSet<char> =
                first_group.intersection(&second_group).copied().collect();
            let second_intersection: HashSet<char> =
                intersection.intersection(&third_group).copied().collect();

            let last_char: char = second_intersection.into_iter().next().unwrap();
            return get_converted_char_code(last_char as usize);
        })
        .sum::<usize>();
    println!("{:?}", result)
}

fn slice_input(input: &str) -> Vec<Vec<String>> {
    let lines = input.lines();
    let mut index = 1;
    let mut result = Vec::new();
    let mut sub_vector = Vec::new();

    for line in lines {
        if index % 3 == 0 {
            sub_vector.push(line.to_owned());
            result.push(sub_vector.clone());
            sub_vector.clear();
        } else {
            sub_vector.push(line.to_owned());
        }
        index += 1;
    }
    return result;
}

fn rucksack_common_char(rucksack: &str) -> usize {
    let (first, second) = rucksack.split_at(rucksack.len() / 2);
    let set: HashSet<char> = first.chars().collect();

    let result = second.chars().find(|c| set.contains(&c)).unwrap();
    let char_code = result as usize;
    return get_converted_char_code(char_code);
}

fn get_converted_char_code(char_code: usize) -> usize {
    const LOWER_CASE_ALPHABET: usize = 97;
    const UPPER_CASE_ALPHABET: usize = 65;
    const UPPER_CASE_MAP_START: usize = 27;
    let converted_code = match char_code {
        65..=90 => char_code - UPPER_CASE_ALPHABET + UPPER_CASE_MAP_START,
        97..=122 => char_code - LOWER_CASE_ALPHABET + 1,
        _ => 0,
    } as usize;
    return converted_code;
}
