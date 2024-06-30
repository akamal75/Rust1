fn longest_prefix(strs: &[&str]) -> String {
    if strs.is_empty() { return "".to_string(); }
    let mut prefix = strs[0].to_string();
    for s in strs {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() { return "".to_string(); }
        }
    }
    prefix
}

fn main() {
    let strs = ["plower", "please", "plate"];
    println!("Longest common prefix: {}", longest_prefix(&strs));
}
