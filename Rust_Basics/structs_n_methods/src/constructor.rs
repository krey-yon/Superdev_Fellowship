impl Rectangle {
    fn square(size: i32) -> Self {
        Self { width: size, height: size }
    }

    fn area(&self) -> i32 {
        self.width * self.height
    }
}
