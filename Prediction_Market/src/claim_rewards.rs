// claim_rewards — Winners burn tokens, receive collateral.
//
// Mirrors the on-chain claim_rewards instruction.
//
// VALIDATION:
//   1. !market.is_settled → Err("MarketNotSettled")
//   2. market.winning_outcome is None → Err("WinningOutcomeNotSet")
//
// LOGIC:
//   payout = match winning_outcome:
//     OutcomeA → user.balance_a
//     OutcomeB → user.balance_b
//
//   if payout > 0:
//     checked_sub total_collateral_locked
//     zero out the winning balance (prevents double-claim)
//
// RETURNS: Ok(payout_amount)
//
fn claim_rewards(
    market: &mut Market,
    user: &mut UserState,
) -> Result<u64, &'static str> {
    // TODO: check market is settled

    // TODO: check winning_outcome is Some

    // TODO: determine payout based on winning side

    // TODO: if payout > 0, update vault and zero out user's winning balance

    // TODO: return Ok(payout)
    todo!()
}