fn sort_sell_orders(orders: &[(u64, u64)]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..orders.len()).collect();

    indices.sort_by(|&a, &b| {
        // Lowest price first, then earliest timestamp
        orders[a].0.cmp(&orders[b].0)
            .then(orders[a].1.cmp(&orders[b].1))
    });

    indices
}
