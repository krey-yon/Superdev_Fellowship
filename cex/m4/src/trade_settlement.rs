fn settle_trades(
    mut buyer_base: u64, mut buyer_quote: u64,
    mut seller_base: u64, mut seller_quote: u64,
    trades: &[(u64, u64)],
) -> (u64, u64, u64, u64) {
    for &(price, qty) in trades {
        let cost = price * qty;

        buyer_base += qty;
        buyer_quote -= cost;
        seller_base -= qty;
        seller_quote += cost;
    }

    (buyer_base, buyer_quote, seller_base, seller_quote)
}
