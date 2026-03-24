fn largest(list: &[i32]) -> i32 {
    let mut max = list[0];
    for &num in list {
        if num > max {
            max = num;
        }
    }
    max
}
