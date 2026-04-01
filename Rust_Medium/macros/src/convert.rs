macro_rules! convert {
    (celsius_to_f, $temp:expr) => {
        $temp * 9 / 5 + 32
    };
    (f_to_celsius, $temp:expr) => {
        ($temp - 32) * 5 / 9
    };
}

fn temp_test(c: i32) -> i32 {
    convert!(celsius_to_f, c)
}
