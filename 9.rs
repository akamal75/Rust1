use std::io;

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let mut input = String::new();
    println!("Enter a value you need  to reverse:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    println!(" The Reversed string is: {}", reverse_string(input));
}