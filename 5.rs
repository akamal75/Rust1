fn median_of_sorted_array(sorted_array: &[i32]) -> f64 {
    let len = sorted_array.len();
    if len % 2 == 0 {
        (sorted_array[len / 2 - 1] + sorted_array[len / 2]) as f64 / 2.0
    } else {
        sorted_array[len / 2] as f64
    }
}

fn main() {
    let sorted_array = [1, 2, 3, 4, 5];
    println!("Median is : {}", median_of_sorted_array(&sorted_array));
}
