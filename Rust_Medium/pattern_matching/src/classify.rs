fn classify(n: i32) -> String {
    match n {
        0 => "zero".to_string(),
        n @ 1..=10 => format!("small: {}", n),
        n @ -10..=-1 => format!("neg small: {}", n),
        n => format!("big: {}", n),
    }
}
