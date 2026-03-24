fn double_all(nums: &[i32]) -> Vec<i32> {
    nums.iter().map(|&x| x * 2).collect()
}
