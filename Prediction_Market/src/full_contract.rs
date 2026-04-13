fn simulate_market(n_users: usize, authority: &str, deadline: i64, ops: &[&str]) -> String {
    let mut market = Market {
        market_id: 1,
        authority: authority.to_string(),
        settlement_deadline: deadline,
        is_settled: false,
        winning_outcome: None,
        total_collateral_locked: 0,
    };
    let mut users: Vec<UserState> = (0..n_users)
        .map(|_| UserState { balance_a: 0, balance_b: 0 })
        .collect();
    let mut claimed = vec![0u64; n_users];

    for op in ops {
        let parts: Vec<&str> = op.split(':').collect();

        if parts[0] == "S" {
            let user: usize = parts[1].parse().unwrap();
            let amount: u64 = parts[2].parse().unwrap();
            let time: i64 = parts[3].parse().unwrap();

            if !market.is_settled && time < market.settlement_deadline && amount > 0 {
                if let Some(new_total) = market.total_collateral_locked.checked_add(amount) {
                    if let Some(new_a) = users[user].balance_a.checked_add(amount) {
                        if let Some(new_b) = users[user].balance_b.checked_add(amount) {
                            market.total_collateral_locked = new_total;
                            users[user].balance_a = new_a;
                            users[user].balance_b = new_b;
                        }
                    }
                }
            }
        } else if parts[0] == "M" {
            let user: usize = parts[1].parse().unwrap();
            let time: i64 = parts[2].parse().unwrap();

            if !market.is_settled && time < market.settlement_deadline {
                let pairs = users[user].balance_a.min(users[user].balance_b);
                if pairs > 0 {
                    if let Some(new_total) = market.total_collateral_locked.checked_sub(pairs) {
                        market.total_collateral_locked = new_total;
                        users[user].balance_a -= pairs;
                        users[user].balance_b -= pairs;
                    }
                }
            }
        } else if parts[0] == "SETTLE" {
            let caller = parts[1];
            let winner = parts[2];
            let time: i64 = parts[3].parse().unwrap();

            if caller == market.authority && !market.is_settled && time <= market.settlement_deadline {
                let parsed = match winner {
                    "A" => Some(WinningOutcome::OutcomeA),
                    "B" => Some(WinningOutcome::OutcomeB),
                    _ => None,
                };

                if let Some(w) = parsed {
                    market.is_settled = true;
                    market.winning_outcome = Some(w);
                }
            }
        } else if parts[0] == "CLAIM" {
            let user: usize = parts[1].parse().unwrap();

            if market.is_settled {
                if let Some(winner) = market.winning_outcome.as_ref() {
                    let payout = match winner {
                        WinningOutcome::OutcomeA => users[user].balance_a,
                        WinningOutcome::OutcomeB => users[user].balance_b,
                    };

                    if payout > 0 {
                        if let Some(new_total) = market.total_collateral_locked.checked_sub(payout) {
                            market.total_collateral_locked = new_total;
                            match winner {
                                WinningOutcome::OutcomeA => users[user].balance_a = 0,
                                WinningOutcome::OutcomeB => users[user].balance_b = 0,
                            }
                            claimed[user] += payout;
                        }
                    }
                }
            }
        }
    }

    let mut result = market.total_collateral_locked.to_string();
    for c in &claimed {
        result.push(' ');
        result.push_str(&c.to_string());
    }
    result
}
