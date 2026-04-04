// run_matching_engine: Processes a stream of orders and generates trades.
//
// INPUT:
//   orders: &[(u8, u64, u64)] → list of orders:
//     .0 = side: 0 = BUY, 1 = SELL
//     .1 = price
//     .2 = quantity
//
// OUTPUT:
//   Vec<(u64, u64)> → trades in execution order: (trade_price, trade_qty)
//
// ALGORITHM:
//   Maintain: bids Vec<(u64,u64)>, asks Vec<(u64,u64)>, trades Vec<(u64,u64)>
//
//   For each (side, price, qty):
//     let mut remaining = qty;
//
//     If BUY (side == 0):
//       asks.sort_by_key(|&(p, _)| p);          // cheapest first
//       While remaining > 0 && !asks.empty() && asks[0].0 <= price:
//         trade_qty = remaining.min(asks[0].1)
//         trades.push((asks[0].0, trade_qty))
//         remaining -= trade_qty; asks[0].1 -= trade_qty
//         if asks[0].1 == 0 { asks.remove(0); }
//       If remaining > 0: bids.push((price, remaining))
//
//     If SELL (side == 1):
//       bids.sort_by(|a, b| b.0.cmp(&a.0));     // highest first
//       While remaining > 0 && !bids.empty() && bids[0].0 >= price:
//         (same logic against bids)
//       If remaining > 0: asks.push((price, remaining))
//
fn run_matching_engine(orders: &[(u8, u64, u64)]) -> Vec<(u64, u64)> {
    let mut bids: Vec<(u64, u64)> = Vec::new();
    let mut asks: Vec<(u64, u64)> = Vec::new();
    let mut trades: Vec<(u64, u64)> = Vec::new();

    for &(side, price, qty) in orders {
        let mut remaining = qty;

        if side == 0 {
            // BUY: match against asks
            asks.sort_by_key(|&(p, _)| p);

            while remaining > 0 && !asks.is_empty() && asks[0].0 <= price {
                let trade_price = asks[0].0;
                let trade_qty = remaining.min(asks[0].1);
                trades.push((trade_price, trade_qty));
                remaining -= trade_qty;
                asks[0].1 -= trade_qty;
                if asks[0].1 == 0 {
                    asks.remove(0);
                }
            }
            if remaining > 0 {
                bids.push((price, remaining));
            }
        } else {
            // SELL: match against bids
            bids.sort_by(|a, b| b.0.cmp(&a.0));

            while remaining > 0 && !bids.is_empty() && bids[0].0 >= price {
                let trade_price = bids[0].0;
                let trade_qty = remaining.min(bids[0].1);
                trades.push((trade_price, trade_qty));
                remaining -= trade_qty;
                bids[0].1 -= trade_qty;
                if bids[0].1 == 0 {
                    bids.remove(0);
                }
            }

            if remaining > 0 {
                asks.push((price, remaining));
            }
        }

    }

    trades
}
