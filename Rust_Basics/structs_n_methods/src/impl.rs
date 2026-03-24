impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}
