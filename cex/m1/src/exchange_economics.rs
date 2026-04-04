fn order_economics(best_bid: u64, best_ask: u64, price: u64, qty: u64, fee_bps: u64) -> (u64, u64, u64, u64) {
    let spread = best_ask - best_bid;
    let midprice = (best_bid + best_ask) / 2;
    let notional = price * qty;
    let fee = notional * fee_bps / 10000;

    (spread, midprice, notional, fee)
}
