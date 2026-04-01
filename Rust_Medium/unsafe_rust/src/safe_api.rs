struct SafeArray {
    data: Vec<i32>,
}

impl SafeArray {
    fn new(data: Vec<i32>) -> Self {
        SafeArray { data }
    }

    fn get(&self, i: usize) -> Option<i32> {
        self.data.get(i).copied()
    }

    unsafe fn get_unchecked(&self, i: usize) -> i32 {
        *self.data.get_unchecked(i)
    }

    fn sum_all(&self) -> i32 {
        let mut sum = 0;
        for i in 0..self.data.len() {
            unsafe {
                sum += self.get_unchecked(i);
            }
        }
        sum
    }
}
