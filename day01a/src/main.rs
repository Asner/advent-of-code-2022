fn main() {
    let result: i32 = include_str!("./input.txt")
        .split("\n\n")
        .map(|line| {
            line.split('\n')
                .map(|n| {
                    if n.is_empty() { return 0 }
                    return n.parse::<i32>().unwrap()
                })
                .sum::<i32>()
        })
        .max().unwrap();

    println!("{result:?}")
}
