fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u32;
    !(2..=sqrt_n).any(|i| n % i == 0)
}

fn main() {
    let number = 17;
    println!("checking whether the number {}  is prime or not :  {}", number, is_prime(number));
}
