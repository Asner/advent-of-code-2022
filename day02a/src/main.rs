use std::collections::HashMap;

fn main() {
    /*
        A = Rock (1)
        B = Paper (2)
        C = Scissors (3)
    */
    let opponent = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    /*
        X = Rock (1)
        Y = Paper (2)
        Z = Scissors (3)
    */
    let my_plays = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let input: i32 = include_str!("./input.txt")
        .lines()
        .map(|strategy| {
            let mut result = strategy.split(' ');
            let oponent_point = opponent.get(&result.next().unwrap()).unwrap();
            let my_point = my_plays.get(&result.next().unwrap()).unwrap();

            let diff = my_point - oponent_point;
            return match diff {
                -2 | 1 => my_point + 6,
                0 => my_point + 3,
                -1 | 2 => my_point + 0,
                _ => 0,
            };
        })
        .sum::<i32>();

    println!("Result: {:?}", input);
}
