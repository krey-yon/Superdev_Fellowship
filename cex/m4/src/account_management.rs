fn process_balance(initial: u64, operations: &[(bool, u64)]) -> u64 {
    let mut balance = initial;
    for &(is_deposit, amount) in operations {
        if is_deposit {
            balance += amount;
        } else {
            if balance >= amount {
                balance -= amount;
            }
        }
    }
    balance
}

fn validate_margin(is_buy: bool, base_bal: u64, quote_bal: u64, price: u64, qty: u64) -> bool {
    if is_buy {
        quote_bal >= price * qty
    } else {
        base_bal >= qty
    }
}
