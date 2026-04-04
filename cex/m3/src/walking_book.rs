fn fill_buy_order(limit_price: u64, mut qty: u64, asks: &[(u64, u64)]) -> (u64, u64, u64) {
    let mut filled = 0u64;
    let mut cost = 0u64;

    for &(ask_price, available) in asks {
        if qty == 0 {
            break;
        }

        if limit_price > 0 && ask_price > limit_price {
            break;
        }

        let fill = qty.min(available);
        filled += fill;
        cost += fill * ask_price;
        qty -= fill;
    }

    (filled, cost, qty)
}
