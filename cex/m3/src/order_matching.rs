fn try_match(buy_price: u64, buy_qty: u64, sell_price: u64, sell_qty: u64) -> (bool, u64, u64) {
    if buy_price >= sell_price {
        let trade_qty = buy_qty.min(sell_qty);
        let trade_price = sell_price;
        (true, trade_price, trade_qty)
    } else {
        (false, 0, 0)
    }
}
