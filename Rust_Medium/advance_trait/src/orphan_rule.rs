use std::fmt;

struct CommaSeparated(Vec<i32>);

impl fmt::Display for CommaSeparated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.0.iter().peekable();
        while let Some(n) = iter.next() {
            write!(f, "{}", n)?;
            if iter.peek().is_some() {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}

fn format_list(nums: Vec<i32>) -> String {
    CommaSeparated(nums).to_string()
}
