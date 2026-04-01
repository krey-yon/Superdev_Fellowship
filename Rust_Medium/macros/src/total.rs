macro_rules! sum {
    () => {
        0
    };
    ($($x:expr),+ $(,)?) => {
        0 $(+ $x)+
    };
}

fn total(a: i32, b: i32, c: i32) -> i32 {
    sum!(a, b, c)
}
