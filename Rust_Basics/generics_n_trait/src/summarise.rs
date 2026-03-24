struct Numbers { data: Vec<i32> }
struct Sentence { words: Vec<String> }

impl Summarize for Numbers {
    type Output = i32;

    fn summarize(&self) -> Self::Output {
        self.data.iter().sum()
    }
}

impl Summarize for Sentence {
    type Output = String;

    fn summarize(&self) -> Self::Output {
        self.words.join(" ")
    }
}
