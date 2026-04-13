fn set_winning_side(
    market: &mut Market,
    caller: &str,
    winner: &str,
    current_time: i64,
) -> Result<(), &'static str> {
    if caller != market.authority {
        return Err("NotAuthority");
    }

    if market.is_settled {
        return Err("MarketAlreadySettled");
    }

    if current_time > market.settlement_deadline {
        return Err("MarketExpired");
    }

    let parsed_winner = match winner {
        "A" => WinningOutcome::OutcomeA,
        "B" => WinningOutcome::OutcomeB,
        _ => return Err("InvalidWinningOutcome"),
    };

    market.is_settled = true;
    market.winning_outcome = Some(parsed_winner);

    Ok(())
}
