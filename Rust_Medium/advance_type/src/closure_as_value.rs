fn make_multiplier(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x * n)
}

fn compose(
    f: Box<dyn Fn(i32) -> i32>,
    g: Box<dyn Fn(i32) -> i32>,
) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| f(g(x)))
}
