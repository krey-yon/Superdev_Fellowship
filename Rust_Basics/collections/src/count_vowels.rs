fn count_vowels(s: &str) -> usize {
    s.to_lowercase()
        .chars()
        .filter(|&c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}
