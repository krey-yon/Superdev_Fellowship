fn initialize_market(
    market_id: u32,
    authority: &str,
    settlement_deadline: i64,
    current_time: i64,
) -> Result<Market, &'static str> {
    if settlement_deadline <= current_time {
        return Err("InvalidSettlementDeadline");
    }

    Ok(Market {
        authority: authority.to_string(),
        market_id,
        settlement_deadline,
        is_settled: false,
        winning_outcome: None,
        total_collateral_locked: 0,
    })
}
