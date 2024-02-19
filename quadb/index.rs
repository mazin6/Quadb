fn first_occurrence_index(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    
    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            // Check if this is the first occurrence
            if mid == 0 || arr[mid - 1] != target {
                return Some(mid);
            } else {
                // Search on the left side for earlier occurrence
                right = mid - 1;
            }
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    
    None
}

fn main() {
    let arr = [1, 2, 3, 4, 4, 4, 5, 6, 7];
    let target = 4;
    
    match first_occurrence_index(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} is not present in the array", target),
    }
}
