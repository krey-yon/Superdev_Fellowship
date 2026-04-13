fn merge_tokens(
    market: &mut Market,
    user: &mut UserState,
    current_time: i64,
) -> Result<u64, &'static str> {
    if market.is_settled {
        return Err("MarketAlreadySettled");
    }

    if current_time >= market.settlement_deadline {
        return Err("MarketExpired");
    }

    let pairs = user.balance_a.min(user.balance_b);

    if pairs == 0 {
        return Err("InvalidAmount");
    }

    market.total_collateral_locked = market
        .total_collateral_locked
        .checked_sub(pairs)
        .ok_or("MathOverflow")?;

    user.balance_a = user.balance_a.checked_sub(pairs).ok_or("MathOverflow")?;
    user.balance_b = user.balance_b.checked_sub(pairs).ok_or("MathOverflow")?;

    Ok(pairs)
}
