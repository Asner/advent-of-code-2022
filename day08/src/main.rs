
fn main() {
    let input = include_str!("./input.txt");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    let result_map = build_tree_map(input);
    let mut visible_trees :u32 = 0;
    let max_height: usize = result_map.len() - 1;
    let max_width: usize = result_map[0].len() - 1;

    for (i, row) in result_map.iter().enumerate() {
        for (x, _tree_height) in row.iter().enumerate() {
            if x == 0 || (x == max_width) || i == 0 || (i == max_height) {
                visible_trees += 1;
                continue;
            }
            let row_result = look_row(row, &x);
            if row_result {
                visible_trees += 1;
                continue;
            }
            let transponse = transponse(&x, &result_map);
            assert!(transponse[i].eq(_tree_height), "Must match");
            let column_result = look_row(&transponse, &i);
            if column_result {
                visible_trees += 1;
            }
        }
    }

    println!("Part 1 - Visible trees {:?}", visible_trees);
}

fn part_two(input:&str) {
    let result_map = build_tree_map(input);
    let mut max_scenic_score :u32 = 0;

    for (i, row) in result_map.iter().enumerate() {
        for (x, _tree_height) in row.iter().enumerate() {
            let row_result = look_row_count(row, &x);
            let transponse = transponse(&x, &result_map);
            assert!(transponse[i].eq(_tree_height), "Must match");
            let column_result = look_row_count(&transponse, &i);

            let scenic_result = row_result * column_result;
            if max_scenic_score.lt(&scenic_result) {
                max_scenic_score = scenic_result
            }
        }
    }
    println!("Part two - Max scenic score {}", max_scenic_score);
}

fn build_tree_map(input: &str) -> Vec<Vec<u32>>{

    let mut result: Vec<Vec<u32>> = Vec::new();

    let mut stack_split = input.lines();
    while let Some(line) = stack_split.next() {
        let result_row = line.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        result.push(result_row);
    }
    result
}

fn look_row(row: &Vec<u32>, row_index: &usize) -> bool {
    let row_value = row.get(*row_index).unwrap();
    let mut left_look = true;
    let mut right_look = true;
    for (i, tree_height) in row.iter().enumerate() {
        let is_left = i.lt(row_index);
        if i.eq(row_index) {
            continue
        };
        if is_left {
            if tree_height.ge(row_value) {
                left_look = false;
            }
        } else {
            if tree_height.ge(row_value) {
                right_look = false;
                break;
            }
        }
    }
    left_look || right_look
}

fn look_row_count(row: &Vec<u32>, row_index: &usize) -> u32 {
    let row_value = row.get(*row_index).unwrap();
    let mut left_look_count = 0;
    let mut right_look_count = 0;
    for (i, tree_height) in row.iter().enumerate() {
        let is_left = i.lt(row_index);
        if i.eq(row_index) {
            continue
        };
        if is_left {
            if tree_height.ge(row_value) {
                left_look_count = 0
            }
            left_look_count += 1;
        } else {
            if tree_height.ge(row_value) {
                right_look_count += 1;
                break;
            }
            right_look_count += 1;
        }
    }
    left_look_count * right_look_count
}


fn transponse(row_index: &usize, tree_map: &Vec<Vec<u32>>) -> Vec<u32> {
     (0..tree_map.len())
        .map(|i| tree_map[i][*row_index])
        .collect::<Vec<u32>>()
}