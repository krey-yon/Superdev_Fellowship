struct Countdown {
    n: i32,
}

impl Countdown {
    fn new(n: i32) -> Self {
        Countdown { n }
    }
}

impl Iterator for Countdown {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
