fn shortest_word(s: &str) -> &str {
    s.split_whitespace().min_by_key(|w| w.len()).unwrap()
}

fn main() {
    let input = " I am From LPU ";
    println!("Shortest word from these are: {}", shortest_word(input));
}

