fn kth_smallest(arr: &[i32], k: usize) -> i32 {
    let mut sorted = arr.to_vec();
    sorted.sort();
    sorted[k - 1]
}

fn main() {
    let arr = [3, 1, 2, 4, 5];
    println!("3rd smallest element from the list is : {}", kth_smallest(&arr, 3));
}