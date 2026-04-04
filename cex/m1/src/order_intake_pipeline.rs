fn process_incoming_order(
    order_type: &str,
    side: &str,
    price: u64,
    qty: u64,
    best_bid: u64,
    best_ask: u64,
) -> &'static str {
    // --- STEP 1: Validation ---
    if order_type != "MARKET" && order_type != "LIMIT" {
        return "REJECTED";
    }

    if side != "BUY" && side != "SELL" {
        return "REJECTED";
    }

    if qty == 0 {
        return "REJECTED";
    }

    if order_type == "LIMIT" && price == 0 {
        return "REJECTED";
    }

    // --- STEP 2: Classification ---
    if order_type == "MARKET" {
        return "IMMEDIATE";
    }

    if side == "BUY" && price >= best_ask {
        return "IMMEDIATE";
    }

    if side == "SELL" && price <= best_bid {
        return "IMMEDIATE";
    }

    "RESTING"
}
