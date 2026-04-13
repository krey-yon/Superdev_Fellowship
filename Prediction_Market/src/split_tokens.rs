fn split_tokens(
    market: &mut Market,
    user: &mut UserState,
    amount: u64,
    current_time: i64,
) -> Result<(), &'static str> {
    if market.is_settled {
        return Err("MarketAlreadySettled");
    }

    if current_time >= market.settlement_deadline {
        return Err("MarketExpired");
    }

    if amount == 0 {
        return Err("InvalidAmount");
    }

    market.total_collateral_locked = market
        .total_collateral_locked
        .checked_add(amount)
        .ok_or("MathOverflow")?;

    user.balance_a = user
        .balance_a
        .checked_add(amount)
        .ok_or("MathOverflow")?;

    user.balance_b = user
        .balance_b
        .checked_add(amount)
        .ok_or("MathOverflow")?;

    Ok(())
}
