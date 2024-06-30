fn count_occurrences(arr: &[i32], target: i32) -> usize {
    arr.iter().filter(|&&x| x == target).count()
}

fn main() {
    let arr = [1, 2, 2, 4, 5, 2, 2,2,3,3,3,4,5,5,5,5,];
    let target = 3;
    let occurrence_count = count_occurrences(&arr, target);
    println!("Number of occurrences of number {}: {}", target, occurrence_count);
}
