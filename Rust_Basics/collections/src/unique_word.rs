use std::collections::HashSet;

fn unique_word_count(s: &str) -> usize {
    s.split_whitespace().collect::<HashSet<&str>>().len()
}
