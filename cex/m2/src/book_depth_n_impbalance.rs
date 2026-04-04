fn book_snapshot(
    bid_levels: &[(u64, u64)],
    ask_levels: &[(u64, u64)],
    depth: usize,
) -> (Vec<(u64, u64)>, Vec<(u64, u64)>, u64) {
    // Step 1: Take the top 'depth' levels from each side
    let top_bids: Vec<(u64, u64)> = bid_levels.iter().take(depth).copied().collect();
    let top_asks: Vec<(u64, u64)> = ask_levels.iter().take(depth).copied().collect();

    // Step 2: Sum quantities on each side
    let bid_qty: u64 = top_bids.iter().map(|(_, q)| *q).sum();
    let ask_qty: u64 = top_asks.iter().map(|(_, q)| *q).sum();

    // Step 3: Calculate imbalance
    let imbalance: u64 = if bid_qty + ask_qty == 0 {
        0
    } else {
        bid_qty * 100 / (bid_qty + ask_qty)
    };

    (top_bids, top_asks, imbalance)
}
