macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

fn compute(n: i32) -> i32 {
    square!(n)
}
