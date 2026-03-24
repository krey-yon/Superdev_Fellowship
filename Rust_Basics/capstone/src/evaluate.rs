fn evaluate(expr: &str) -> Result<i32, String> {
    let parts: Vec<&str> = expr.split_whitespace().collect();

    if parts.len() != 3 {
        return Err("invalid expression".to_string());
    }

    let left = parts[0].parse::<i32>().map_err(|_| "invalid expression".to_string())?;
    let operator = parts[1];
    let right = parts[2].parse::<i32>().map_err(|_| "invalid expression".to_string())?;

    match operator {
        "+" => Ok(left + right),
        "-" => Ok(left - right),
        "*" => Ok(left * right),
        "/" => {
            if right == 0 {
                Err("division by zero".to_string())
            } else {
                Ok(left / right)
            }
        }
        _ => Err("unknown operator".to_string()),
    }
}
