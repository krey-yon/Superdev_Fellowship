trait Formatter {
    fn format(&self, input: &str) -> String;
}

struct Upper;
struct Snake;
struct Trim;

impl Formatter for Upper {
    fn format(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

impl Formatter for Snake {
    fn format(&self, input: &str) -> String {
        input.split_whitespace().collect::<Vec<_>>().join("_")
    }
}

impl Formatter for Trim {
    fn format(&self, input: &str) -> String {
        input.trim().to_string()
    }
}

fn apply_all(input: &str, fmts: &[Box<dyn Formatter>]) -> String {
    fmts.iter().fold(input.to_string(), |acc, f| f.format(&acc))
}
