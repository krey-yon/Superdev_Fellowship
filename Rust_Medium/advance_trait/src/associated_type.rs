trait Summary {
    type Output;
    fn summarize(&self) -> Self::Output;
}

struct Numbers { data: Vec<i32> }
struct Words { data: Vec<String> }

impl Summary for Numbers {
    type Output = i32;
    fn summarize(&self) -> Self::Output {
        self.data.iter().sum()
    }
}

impl Summary for Words {
    type Output = String;
    fn summarize(&self) -> Self::Output {
        self.data.join(" ")
    }
}
