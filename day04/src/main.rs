fn main() {
    assert!((3..5).contains(&4));
    let input = include_str!("./input.txt");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    let result: u32 = input
        .lines()
        .map(|line| {
            let line: _ = line
                .split(',')
                .map(|assignments| {
                    let assignment: Vec<_> = assignments.split('-').collect();
                    let first = assignment[0].parse::<u32>().unwrap();
                    let second = assignment[1].parse::<u32>().unwrap();

                    return (first, second);
                })
                .collect::<Vec<_>>();
            let first_assigment = line.get(0).unwrap();
            let second_assignment = line.get(1).unwrap();

            let first_range = first_assigment.0..=first_assigment.1;
            let second_range = second_assignment.0..=second_assignment.1;

            return (first_range.contains(&second_assignment.0)
                && first_range.contains(&second_assignment.1)
                || second_range.contains(&first_assigment.0)
                    && second_range.contains(&first_assigment.1)) as u32;
        })
        .sum::<u32>();
    println!("{:?}", result)
}

fn part_two(input: &str) {
    let result: u32 = input
        .lines()
        .map(|line| {
            let line: _ = line
                .split(',')
                .map(|assignments| {
                    let assignment: Vec<_> = assignments.split('-').collect();
                    let first = assignment[0].parse::<u32>().unwrap();
                    let second = assignment[1].parse::<u32>().unwrap();

                    return (first, second);
                })
                .collect::<Vec<_>>();
            let first_assigment = line.get(0).unwrap();
            let second_assignment = line.get(1).unwrap();

            let first_range = first_assigment.0..=first_assigment.1;
            let second_range = second_assignment.0..=second_assignment.1;

            return (first_range.contains(&second_assignment.0)
                || first_range.contains(&second_assignment.1)
                    && second_range.contains(&first_assigment.0)
                || second_range.contains(&first_assigment.1)) as u32;
        })
        .sum::<u32>();
    println!("{:?}", result)
}
