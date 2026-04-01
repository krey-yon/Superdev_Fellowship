fn parse_command(input: &str) -> String {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    match tokens.as_slice() {
        ["quit"] => "Goodbye".to_string(),
        ["echo", rest @ ..] => rest.join(" "),
        ["add", x, y] => {
            let sum: i32 = x.parse().unwrap() + y.parse().unwrap();
            sum.to_string()
        }
        ["repeat", n, msg @ ..] => {
            let count: usize = n.parse().unwrap();
            std::iter::repeat(msg.join(" "))
                .take(count)
                .collect::<Vec<_>>()
                .join(" ")
        }
        _ => "Unknown".to_string(),
    }
}
