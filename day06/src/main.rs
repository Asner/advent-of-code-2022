fn main() {
    let input = include_str!("./input.txt");
    println!("Marker found for part 1: {} ", find_marker(input, 4));
    println!("Marker found for part 2: {} ", find_marker(input, 14));
}

fn find_marker(input: &str, chunk_size: usize) -> usize {
    for (i, _c) in input.chars().enumerate() {
        let next_chars: Vec<char> = input[i..i + chunk_size].to_string().chars().collect();
        let mut four_clone = next_chars.clone();
        four_clone.sort_unstable();
        four_clone.dedup();

        let result = next_chars.len() == four_clone.len();

        if result {
            return i + chunk_size;
        }
    }
    return 0;
}
