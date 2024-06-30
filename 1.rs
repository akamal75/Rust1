use std::io;

fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    cleaned.eq_ignore_ascii_case(&cleaned.chars().rev().collect::<String>())
}

fn main() {
    let mut input = String::new();
    println!("Enter you word to check whether its a palindrome:");
    io::stdin().read_line(&mut input).expect("Failed ");
    let input = input.trim();
    println!("  checking whether the given word is a palindrome: {}", is_palindrome(input));
}
