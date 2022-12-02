use std::collections::HashMap;

fn main() {
    /*
        A = Rock (1)
        B = Paper (2)
        C = Scissors (3)
    */
    let opponent_plays = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    /*
        X = Lose (1)
        Y = Draw (2)
        Z = Win (3)
    */
    let expected_results = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let input: i32 = include_str!("./input.txt")
        .lines()
        .map(|strategy| {
            let mut result = strategy.split(' ');
            let oponent_point = opponent_plays.get(&result.next().unwrap()).unwrap();
            let expected_result = expected_results.get(&result.next().unwrap()).unwrap();

            return match expected_result {
                1 => {
                    if ((oponent_point - 1) % 3) == 0 {
                        3
                    } else {
                        (oponent_point - 1) % 3
                    }
                }
                2 => oponent_point + 3,
                3 => {
                    if ((oponent_point + 1) % 3) == 0 {
                        9
                    } else {
                        (oponent_point + 1) % 3 + 6
                    }
                }
                _ => 0,
            };
        })
        .sum::<i32>();

    println!("Result: {:?}", input);
}
