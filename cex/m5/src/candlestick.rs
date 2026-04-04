fn build_candle(trades: &[(u64, u64)]) -> (u64, u64, u64, u64, u64, u64) {
    let open = trades[0].0;
    let close = trades[trades.len() - 1].0;
    let high = trades.iter().map(|(p, _)| *p).max().unwrap();

    let low: u64 = trades.iter().map(|(p, _)| *p).min().unwrap();
    let volume: u64 = trades.iter().map(|(_, q)| *q).sum();

    let total_value: u64 = trades.iter().map(|(p, q)| p * q).sum();
    let vwap: u64 = total_value / volume;

    (open, high, low, close, volume, vwap)
}
