fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];

    for &num in arr.iter().skip(1) {
        current_sum = current_sum.max(current_sum + num);
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

fn main() {
    
    let arr = [7, -5, 5, -7, -2, 1, 5, -3];
    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum is : {}", max_sum);
}