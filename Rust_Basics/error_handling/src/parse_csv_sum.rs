fn parse_csv_sum(s: &str) -> Result<i32, String> {
    if s.is_empty() {
        return Err("empty input".to_string());
    }

    let mut sum = 0;
    for part in s.split(',') {
        let num = part.parse::<i32>().map_err(|_| format!("invalid number: {}", part))?;
        sum += num;
    }
    
    Ok(sum)
}
