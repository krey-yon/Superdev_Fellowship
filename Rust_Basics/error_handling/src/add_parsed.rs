fn add_parsed(a: &str, b: &str) -> Result<i32, String> {
    let num_a = a.parse::<i32>().map_err(|_| "parse error".to_string())?;
    let num_b = b.parse::<i32>().map_err(|_| "parse error".to_string())?;
    
    Ok(num_a + num_b)
}
