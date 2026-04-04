fn analyze_volume(trades: &[(u64, u8)]) -> (u64, u64, u64) {
    let mut buy_vol = 0u64;
    let mut sell_vol = 0u64;

    for &(qty, side) in trades {
        if side == 0 {
            buy_vol += qty;
        } else {
            sell_vol += qty;
        }
    }

    let total = buy_vol + sell_vol;
    let buy_pct: u64 = if total == 0 {
        0
    } else {
        buy_vol * 100 / total
    };

    (buy_vol, sell_vol, buy_pct)
}
