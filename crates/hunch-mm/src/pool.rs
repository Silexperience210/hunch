//! LMSR market-maker pool with a maker fee — the mint-as-market-maker brain.
//!
//! The mint is the counterparty for every 1-click bet: it always quotes a YES/NO price via the
//! Logarithmic Market Scoring Rule, adds a **maker fee** (the operator's rake) on each buy, and
//! tracks the realized rake. Worst-case directional subsidy is bounded by `b·ln2` per market,
//! regardless of how one-sided the flow gets. The fee is pure margin on top of that bound.
//!
//! Pure + unit-tested; the math mirrors `apps/hunch-web/lib/{lmsr,amm}.ts` so the browser quote and
//! the mint's ledger agree.

use serde::{Deserialize, Serialize};

/// Which outcome a bettor is buying.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Side {
    Yes,
    No,
}

/// A market-maker pool for one market. Inventory `q_yes`/`q_no` are sats of payout sold of each side.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
    pub market: String,
    pub q_yes: f64,
    pub q_no: f64,
    /// Liquidity depth (sats). Bigger = deeper book, more capital at risk (subsidy bound `b·ln2`).
    pub b: f64,
    /// Maker fee in basis points (100 bps = 1%) — the operator's rake on each buy.
    pub fee_bps: u32,
    /// Realized rake (sum of maker fees) across all fills (sats).
    pub realized_rake: f64,
    /// Total sats taken in across all fills (sum of `cost`, fee included). Funds settlement payouts.
    #[serde(default)]
    pub collected: f64,
}

/// The financial outcome of settling a market: what the MM took in, what it owes the winners, and
/// its realized profit/loss (which, for a hedged book, is exactly the rake).
#[derive(Debug, Clone, PartialEq)]
pub struct Settlement {
    /// Winning side, or `None` for INVALID (everyone refunded their cost).
    pub winner: Option<Side>,
    /// Total sats the MM collected from all bettors.
    pub total_in: f64,
    /// Sats owed to winners (each winning share pays 1 sat); for INVALID, refunds = total_in.
    pub payout: f64,
    /// MM realized profit/loss = total_in − payout. Bounded below by `rake − b·ln2`.
    pub mm_pnl: f64,
    /// The accrued maker rake (fee component) for reference.
    pub rake: f64,
}

/// A quote for buying `shares` sats of payout on a side.
#[derive(Debug, Clone, PartialEq)]
pub struct Quote {
    pub shares: f64,
    /// Fair LMSR cost before the maker fee (sats).
    pub fair: f64,
    /// Maker fee (operator rake) for this trade (sats).
    pub fee: f64,
    /// Total the bettor pays = fair + fee (sats).
    pub cost: f64,
    /// Average price paid per share (cost / shares), 0..1+fee.
    pub avg_price: f64,
    /// Spot YES price before the trade, 0..1.
    pub price_before: f64,
    /// Spot YES price after the trade, 0..1 (slippage).
    pub price_after: f64,
}

/// Numerically-stable log-sum-exp.
fn lse(a: f64, b: f64) -> f64 {
    let m = a.max(b);
    m + ((a - m).exp() + (b - m).exp()).ln()
}

impl Pool {
    /// A fresh, empty pool (50/50) of depth `b` and fee `fee_bps`.
    pub fn new(market: impl Into<String>, b: f64, fee_bps: u32) -> Self {
        Pool {
            market: market.into(),
            q_yes: 0.0,
            q_no: 0.0,
            b,
            fee_bps,
            realized_rake: 0.0,
            collected: 0.0,
        }
    }

    /// LMSR cost function C(q) = b · ln(e^(qYes/b) + e^(qNo/b)) at given inventory.
    fn cost_at(&self, q_yes: f64, q_no: f64) -> f64 {
        self.b * lse(q_yes / self.b, q_no / self.b)
    }

    fn cost(&self) -> f64 {
        self.cost_at(self.q_yes, self.q_no)
    }

    /// Instantaneous (fair) YES price = implied probability, 0..1.
    pub fn price_yes(&self) -> f64 {
        let m = (self.q_yes / self.b).max(self.q_no / self.b);
        let ey = (self.q_yes / self.b - m).exp();
        let en = (self.q_no / self.b - m).exp();
        ey / (ey + en)
    }

    pub fn price_no(&self) -> f64 {
        1.0 - self.price_yes()
    }

    /// Maker fee as a fraction (e.g. 200 bps → 0.02).
    pub fn fee_rate(&self) -> f64 {
        self.fee_bps as f64 / 10_000.0
    }

    /// The market maker's worst-case directional subsidy (max loss before fees), `b·ln2`.
    pub fn max_subsidy(&self) -> f64 {
        self.b * std::f64::consts::LN_2
    }

    /// Quote buying `shares` sats of payout on `side`: fair LMSR cost + maker fee.
    pub fn quote_buy(&self, side: Side, shares: f64) -> Quote {
        let after = match side {
            Side::Yes => self.cost_at(self.q_yes + shares, self.q_no),
            Side::No => self.cost_at(self.q_yes, self.q_no + shares),
        };
        let fair = after - self.cost();
        let fee = fair * self.fee_rate();
        let cost = fair + fee;
        let price_after = match side {
            Side::Yes => {
                let p = Pool {
                    q_yes: self.q_yes + shares,
                    ..self.clone()
                };
                p.price_yes()
            }
            Side::No => {
                let p = Pool {
                    q_no: self.q_no + shares,
                    ..self.clone()
                };
                p.price_yes()
            }
        };
        Quote {
            shares,
            fair,
            fee,
            cost,
            avg_price: if shares > 0.0 { cost / shares } else { 0.0 },
            price_before: self.price_yes(),
            price_after,
        }
    }

    /// How many shares a `budget` of sats (fee included) buys on `side`. Bisection on `quote_buy.cost`.
    pub fn shares_for_budget(&self, side: Side, budget: f64) -> f64 {
        if budget <= 0.0 {
            return 0.0;
        }
        let (mut lo, mut hi) = (0.0_f64, budget.max(1.0));
        while self.quote_buy(side, hi).cost < budget {
            hi *= 2.0;
        }
        for _ in 0..60 {
            let mid = (lo + hi) / 2.0;
            if self.quote_buy(side, mid).cost < budget {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        (lo + hi) / 2.0
    }

    /// Apply a buy: move inventory, accrue the maker fee, and bank the sats taken in. Returns the
    /// rake earned on this fill.
    pub fn apply_buy(&mut self, side: Side, shares: f64) -> f64 {
        let q = self.quote_buy(side, shares);
        match side {
            Side::Yes => self.q_yes += shares,
            Side::No => self.q_no += shares,
        }
        self.realized_rake += q.fee;
        self.collected += q.cost;
        q.fee
    }

    /// Settle the market for a winning side (or `None` = INVALID). Computes the payout owed to winners
    /// and the MM's realized P&L. For a hedged book (q_yes == q_no) the P&L equals the rake exactly,
    /// regardless of outcome — the operator's edge with zero directional risk.
    pub fn settle(&self, winner: Option<Side>) -> Settlement {
        let payout = match winner {
            Some(Side::Yes) => self.q_yes,
            Some(Side::No) => self.q_no,
            None => self.collected, // INVALID → refund everyone their cost
        };
        Settlement {
            winner,
            total_in: self.collected,
            payout,
            mm_pnl: self.collected - payout,
            rake: self.realized_rake,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-9;
    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn empty_pool_is_5050_and_subsidy_is_b_ln2() {
        let p = Pool::new("m", 10_000.0, 200);
        assert!(close(p.price_yes(), 0.5));
        assert!(close(p.max_subsidy(), 10_000.0 * std::f64::consts::LN_2));
    }

    #[test]
    fn fee_makes_cost_exceed_fair_and_is_the_rake() {
        let p = Pool::new("m", 10_000.0, 200); // 2%
        let q = p.quote_buy(Side::Yes, 100.0);
        assert!(q.cost > q.fair);
        assert!(close(q.fee, q.fair * 0.02));
        assert!(close(q.cost, q.fair + q.fee));
        // zero-fee pool: cost == fair
        let p0 = Pool::new("m", 10_000.0, 0);
        let q0 = p0.quote_buy(Side::Yes, 100.0);
        assert!(close(q0.cost, q0.fair) && close(q0.fee, 0.0));
    }

    #[test]
    fn buying_yes_raises_yes_price() {
        let p = Pool::new("m", 10_000.0, 100);
        let q = p.quote_buy(Side::Yes, 500.0);
        assert!(q.price_after > q.price_before && q.price_after < 1.0);
    }

    #[test]
    fn apply_buy_moves_inventory_and_accrues_rake() {
        let mut p = Pool::new("m", 10_000.0, 200);
        let fee1 = p.apply_buy(Side::Yes, 100.0);
        assert!(fee1 > 0.0 && close(p.realized_rake, fee1));
        assert!(p.q_yes > 0.0 && p.price_yes() > 0.5);
        let fee2 = p.apply_buy(Side::No, 100.0);
        assert!(close(p.realized_rake, fee1 + fee2));
    }

    #[test]
    fn shares_for_budget_inverts_cost() {
        let p = Pool::new("m", 10_000.0, 200);
        let shares = p.shares_for_budget(Side::Yes, 100.0);
        assert!(close(p.quote_buy(Side::Yes, shares).cost, 100.0));
        assert_eq!(p.shares_for_budget(Side::Yes, 0.0), 0.0);
    }

    #[test]
    fn hedged_book_settles_to_exactly_the_rake() {
        // Equal shares on each side → q_yes == q_no → zero directional risk: the MM keeps the rake
        // whichever way it resolves. This is the headline economic guarantee.
        let mut p = Pool::new("m", 10_000.0, 200);
        p.apply_buy(Side::Yes, 500.0);
        p.apply_buy(Side::No, 500.0);
        assert!(close(p.q_yes, p.q_no));
        let sy = p.settle(Some(Side::Yes));
        let sn = p.settle(Some(Side::No));
        assert!(close(sy.mm_pnl, p.realized_rake));
        assert!(close(sn.mm_pnl, p.realized_rake));
        assert!(p.realized_rake > 0.0);
    }

    #[test]
    fn settlement_conserves_and_pnl_is_bounded() {
        let mut p = Pool::new("m", 10_000.0, 200);
        p.apply_buy(Side::Yes, 1873.0);
        p.apply_buy(Side::No, 800.0); // unbalanced → outcome-dependent P&L
        for w in [Some(Side::Yes), Some(Side::No)] {
            let s = p.settle(w);
            assert!(close(s.total_in, s.payout + s.mm_pnl)); // money is conserved
            assert!(s.mm_pnl >= p.realized_rake - p.max_subsidy() - 1.0); // bounded downside
        }
    }

    #[test]
    fn invalid_refunds_everyone_with_zero_pnl() {
        let mut p = Pool::new("m", 10_000.0, 200);
        p.apply_buy(Side::Yes, 500.0);
        let s = p.settle(None);
        assert!(close(s.payout, s.total_in) && close(s.mm_pnl, 0.0));
    }

    #[test]
    fn symmetric_at_5050() {
        let p = Pool::new("m", 10_000.0, 150);
        let qy = p.quote_buy(Side::Yes, 250.0);
        let qn = p.quote_buy(Side::No, 250.0);
        assert!(close(qy.cost, qn.cost) && close(qy.fee, qn.fee));
        let _ = EPS;
    }
}
