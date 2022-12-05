use std::borrow::Borrow;

fn main() {
    let input = include_str!("./input.txt");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    let something: Vec<&str> = input.split("\n\n").collect();
    let stacks: Vec<Vec<String>> = build_stacks(something.get(0).unwrap());
    let moves: Vec<Vec<usize>> = build_moves(something.get(1).unwrap());
    let result_crates = move_crates(stacks, moves);
    build_final_message(result_crates);
}

fn part_two(input: &str) {
    let something: Vec<&str> = input.split("\n\n").collect();
    let stacks: Vec<Vec<String>> = build_stacks(something.get(0).unwrap());
    let moves: Vec<Vec<usize>> = build_moves(something.get(1).unwrap());
    let result_crates = move_crates_multiple(stacks, moves);
    build_final_message(result_crates);
}

fn build_stacks(stacks_input: &str) -> Vec<Vec<String>> {
    let clean_stacks = stacks_input
        .replace("    ", " ")
        .replace("[", "")
        .replace("]", "");
    let mut result: Vec<Vec<String>> = Vec::new();

    let mut stack_split = clean_stacks.split("\n").peekable();
    while let Some(line) = stack_split.next() {
        if stack_split.peek().is_some() {
            let temp_string = line
                .split(" ")
                .map(|stack| stack.to_string())
                .collect::<Vec<String>>();
            result.push(temp_string)
        }
    }
    // Transponse
    return (0..result[0].len())
        .map(|i| {
            let mut prev_result = result
                .iter()
                .filter(|vector| !vector[i].is_empty())
                .map(|vector| vector[i].clone())
                .collect::<Vec<String>>();
            prev_result.reverse();
            return prev_result;
        })
        .collect::<Vec<Vec<String>>>();
}

fn build_moves(moves_input: &str) -> Vec<Vec<usize>> {
    return moves_input
        .lines()
        .map(|line| {
            let moves: Vec<usize> = line
                .split(" ")
                .map(|chunk| chunk.parse::<i32>())
                .filter(|chunk| chunk.is_ok())
                .map(|chunk| chunk.unwrap() as usize)
                .collect();
            return moves;
        })
        .collect();
}

fn move_crates(mut stacks: Vec<Vec<String>>, moves: Vec<Vec<usize>>) -> Vec<Vec<String>> {
    for current_move in moves {
        let count = current_move.get(0).unwrap().to_owned();
        let start = current_move.get(1).unwrap().to_owned() - 1;
        let end = current_move.get(2).unwrap().to_owned() - 1;

        for _i in 0..count {
            let current_crate = stacks[start].pop().unwrap();
            let end_stack = &mut stacks[end];
            end_stack.push(current_crate);
        }
    }
    return stacks;
}

fn move_crates_multiple(mut stacks: Vec<Vec<String>>, moves: Vec<Vec<usize>>) -> Vec<Vec<String>> {
    for current_move in moves {
        let count = current_move.get(0).unwrap().to_owned();
        let start = current_move.get(1).unwrap().to_owned() - 1;
        let end = current_move.get(2).unwrap().to_owned() - 1;

        let mut move_vector = Vec::new();
        for _i in 0..count {
            let current_crate = stacks[start].pop().unwrap();
            move_vector.push(current_crate);
        }
        move_vector.reverse();
        stacks[end].extend_from_slice(move_vector.borrow());
    }
    return stacks;
}

fn build_final_message(stacks: Vec<Vec<String>>) {
    let mut message = Vec::new();
    for stack in stacks {
        message.push(stack.last().unwrap().to_owned().to_string())
    }
    println!("Stack {:?}", message.join(""));
}
