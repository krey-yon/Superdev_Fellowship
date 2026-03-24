fn first_word(s: &str) -> String {
    s.split_whitespace().next().unwrap_or(s).to_string()
}
