use std::thread;

fn parallel_sum(nums: Vec<i32>) -> i32 {
    let mid = nums.len() / 2;
    let left = nums[..mid].to_vec();
    let right = nums[mid..].to_vec();

    let left_handle = thread::spawn(move || left.iter().sum::<i32>());
    let right_handle = thread::spawn(move || right.iter().sum::<i32>());

    let left_sum = left_handle.join().unwrap();
    let right_sum = right_handle.join().unwrap();

    left_sum + right_sum
}
