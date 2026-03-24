fn evens_only(nums: &[i32]) -> Vec<i32> {
    nums.iter().copied().filter(|&x| x % 2 == 0).collect()
}
