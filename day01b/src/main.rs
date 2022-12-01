fn main() {
    let mut vec_sums: Vec<i32> = include_str!("./input.txt")
        .split("\n\n")
        .map(|line| {
            line.split('\n')
                .map(|n| {
                    if n.is_empty() {
                        return 0;
                    }
                    return n.parse::<i32>().unwrap();
                })
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    vec_sums.sort();

    let result: i32 = vec_sums.into_iter().rev().take(3).sum::<i32>();

    println!("{:?}", result);
}
