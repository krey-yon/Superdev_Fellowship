fn aggregate_and_best(orders: &[(u64, u64)], is_buy: bool) -> (Vec<(u64, u64)>, u64) {
    let mut map = BTreeMap::new();

    for &(price, qty) in orders {
        *map.entry(price).or_insert(0u64) += qty;
    }

    let levels: Vec<(u64, u64)> = if is_buy {
        map.into_iter().rev().collect()
    } else {
        map.into_iter().collect()
    };

    let best_price = levels[0].0;

    (levels, best_price)
}
