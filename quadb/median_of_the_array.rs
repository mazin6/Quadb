fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (arr[mid_left] as f64 + arr[mid_right] as f64) / 2.0
    } else {
        let mid = len / 2;
        arr[mid] as f64
    }
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5, 6];
    
    println!("Median of arr1: {}", find_median(&arr1)); // Output: 3
    println!("Median of arr2: {}", find_median(&arr2)); // Output: 3.5
}
