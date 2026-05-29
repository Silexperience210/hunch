//! Tier-2 P2P matching engine — pure and synchronous, unit-tested without a network.
//!
//! The matcher does NOT settle or custody anything (Hunch has no central engine). It reads
//! signed kind:38888 orders for a market and proposes compatible pairs; the parties then execute
//! through the mint (HIP-3). Two match types:
//!
//! - **Direct**: a YES bid crosses a YES ask (or NO bid × NO ask) when `bid.price >= ask.price`.
//!   The buyer takes existing tokens from the seller. Execution price is the ask (maker) price.
//! - **Complementary**: a YES bid and a NO bid mint a fresh token pair when their prices sum to at
//!   least `face_value` (the per-token settlement payout, a market/mint parameter from HIP-3).
//!   Their combined sat fully collateralize `amount` units; each receives the matching token.
//!
//! Policy: self-trades (same author on both legs) are skipped; direct matches are taken before
//! complementary ones; matching is greedy on best price with partial fills.

use hunch_protocol::order::{Order, OrderKind, OrderSide};

/// An order as seen on the wire: the protocol [`Order`] plus its Nostr author and event id.
#[derive(Debug, Clone)]
pub struct BookOrder {
    pub author: String,
    pub event_id: String,
    pub order: Order,
}

/// A proposed match between two orders. References orders by event id + author.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Match {
    /// Buyer takes `side` tokens from seller at `price` sat/token.
    Direct {
        side: OrderSide,
        amount: u64,
        price: u64,
        buyer: String,
        seller: String,
        buy_order: String,
        sell_order: String,
    },
    /// A YES buyer and a NO buyer jointly mint `amount` token pairs; `price_yes + price_no >= face`.
    Complementary {
        amount: u64,
        price_yes: u64,
        price_no: u64,
        yes_buyer: String,
        no_buyer: String,
        yes_order: String,
        no_order: String,
    },
}

struct Working {
    author: String,
    event_id: String,
    side: OrderSide,
    kind: OrderKind,
    price: u64,
    remaining: u64,
}

/// Computes proposed matches from a set of orders for a single market.
///
/// `face_value` is the per-token settlement payout (sat) used for complementary matching.
/// `now` is the current unix time; orders with `expires <= now` are ignored.
pub fn match_book(orders: &[BookOrder], face_value: u64, now: u64) -> Vec<Match> {
    let mut work: Vec<Working> = orders
        .iter()
        .filter(|b| b.order.amount > 0 && b.order.expires > now)
        .map(|b| Working {
            author: b.author.clone(),
            event_id: b.event_id.clone(),
            side: b.order.side,
            kind: b.order.kind,
            price: b.order.price,
            remaining: b.order.amount,
        })
        .collect();

    let mut matches = Vec::new();
    match_direct(&mut work, OrderSide::Yes, &mut matches);
    match_direct(&mut work, OrderSide::No, &mut matches);
    match_complementary(&mut work, face_value, &mut matches);
    matches
}

/// Direct same-side crossing: best bid (highest price) vs best ask (lowest price).
fn match_direct(work: &mut [Working], side: OrderSide, matches: &mut Vec<Match>) {
    let mut bids: Vec<usize> = indices(work, side, OrderKind::Bid);
    let mut asks: Vec<usize> = indices(work, side, OrderKind::Ask);
    bids.sort_by(|&a, &b| work[b].price.cmp(&work[a].price)); // price desc
    asks.sort_by(|&a, &b| work[a].price.cmp(&work[b].price)); // price asc

    for &b in &bids {
        for &a in &asks {
            if work[b].remaining == 0 {
                break;
            }
            if work[a].remaining == 0 || work[a].author == work[b].author {
                continue;
            }
            // Asks are ascending: once the cheapest remaining ask exceeds the bid, none qualify.
            if work[b].price < work[a].price {
                break;
            }
            let amount = work[b].remaining.min(work[a].remaining);
            let price = work[a].price;
            matches.push(Match::Direct {
                side,
                amount,
                price,
                buyer: work[b].author.clone(),
                seller: work[a].author.clone(),
                buy_order: work[b].event_id.clone(),
                sell_order: work[a].event_id.clone(),
            });
            work[b].remaining -= amount;
            work[a].remaining -= amount;
        }
    }
}

/// Complementary minting: a YES bid and a NO bid whose prices sum to at least `face_value`.
fn match_complementary(work: &mut [Working], face_value: u64, matches: &mut Vec<Match>) {
    let mut yes_bids: Vec<usize> = work
        .iter()
        .enumerate()
        .filter(|(_, w)| w.side == OrderSide::Yes && w.kind == OrderKind::Bid)
        .map(|(i, _)| i)
        .collect();
    let mut no_bids: Vec<usize> = work
        .iter()
        .enumerate()
        .filter(|(_, w)| w.side == OrderSide::No && w.kind == OrderKind::Bid)
        .map(|(i, _)| i)
        .collect();
    yes_bids.sort_by(|&a, &b| work[b].price.cmp(&work[a].price));
    no_bids.sort_by(|&a, &b| work[b].price.cmp(&work[a].price));

    for &y in &yes_bids {
        for &n in &no_bids {
            if work[y].remaining == 0 {
                break;
            }
            if work[n].remaining == 0 || work[n].author == work[y].author {
                continue;
            }
            // NO bids descending: if the highest remaining NO price can't reach face, none can.
            if work[y].price + work[n].price < face_value {
                break;
            }
            let amount = work[y].remaining.min(work[n].remaining);
            matches.push(Match::Complementary {
                amount,
                price_yes: work[y].price,
                price_no: work[n].price,
                yes_buyer: work[y].author.clone(),
                no_buyer: work[n].author.clone(),
                yes_order: work[y].event_id.clone(),
                no_order: work[n].event_id.clone(),
            });
            work[y].remaining -= amount;
            work[n].remaining -= amount;
        }
    }
}

fn indices(work: &[Working], side: OrderSide, kind: OrderKind) -> Vec<usize> {
    work.iter()
        .enumerate()
        .filter(|(_, w)| w.side == side && w.kind == kind)
        .map(|(i, _)| i)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn order(side: OrderSide, kind: OrderKind, amount: u64, price: u64) -> Order {
        Order { market: "m".into(), side, amount, price, kind, expires: 1_000_000 }
    }

    fn bo(author: &str, side: OrderSide, kind: OrderKind, amount: u64, price: u64) -> BookOrder {
        BookOrder { author: author.into(), event_id: format!("ev-{author}-{price}"), order: order(side, kind, amount, price) }
    }

    #[test]
    fn direct_match_crosses_and_fills_at_ask_price() {
        let orders = vec![
            bo("alice", OrderSide::Yes, OrderKind::Bid, 10_000, 70),
            bo("bob", OrderSide::Yes, OrderKind::Ask, 6_000, 65),
        ];
        let m = match_book(&orders, 100, 0);
        assert_eq!(m.len(), 1);
        match &m[0] {
            Match::Direct { amount, price, buyer, seller, side, .. } => {
                assert_eq!(*amount, 6_000); // min of 10k bid / 6k ask
                assert_eq!(*price, 65); // ask (maker) price
                assert_eq!(buyer, "alice");
                assert_eq!(seller, "bob");
                assert_eq!(*side, OrderSide::Yes);
            }
            other => panic!("expected Direct, got {other:?}"),
        }
    }

    #[test]
    fn no_direct_match_when_bid_below_ask() {
        let orders = vec![
            bo("alice", OrderSide::Yes, OrderKind::Bid, 1_000, 60),
            bo("bob", OrderSide::Yes, OrderKind::Ask, 1_000, 65),
        ];
        assert!(match_book(&orders, 100, 0).is_empty());
    }

    #[test]
    fn self_trade_is_skipped() {
        let orders = vec![
            bo("alice", OrderSide::Yes, OrderKind::Bid, 1_000, 70),
            bo("alice", OrderSide::Yes, OrderKind::Ask, 1_000, 65),
        ];
        assert!(match_book(&orders, 100, 0).is_empty());
    }

    #[test]
    fn complementary_match_when_prices_sum_to_face() {
        let orders = vec![
            bo("alice", OrderSide::Yes, OrderKind::Bid, 5_000, 60),
            bo("bob", OrderSide::No, OrderKind::Bid, 8_000, 45),
        ];
        let m = match_book(&orders, 100, 0);
        assert_eq!(m.len(), 1);
        match &m[0] {
            Match::Complementary { amount, price_yes, price_no, yes_buyer, no_buyer, .. } => {
                assert_eq!(*amount, 5_000);
                assert_eq!(*price_yes, 60);
                assert_eq!(*price_no, 45); // 60 + 45 = 105 >= 100
                assert_eq!(yes_buyer, "alice");
                assert_eq!(no_buyer, "bob");
            }
            other => panic!("expected Complementary, got {other:?}"),
        }
    }

    #[test]
    fn no_complementary_when_sum_below_face() {
        let orders = vec![
            bo("alice", OrderSide::Yes, OrderKind::Bid, 5_000, 40),
            bo("bob", OrderSide::No, OrderKind::Bid, 5_000, 45), // 85 < 100
        ];
        assert!(match_book(&orders, 100, 0).is_empty());
    }

    #[test]
    fn expired_orders_are_ignored() {
        let mut a = bo("alice", OrderSide::Yes, OrderKind::Bid, 1_000, 70);
        a.order.expires = 500;
        let b = bo("bob", OrderSide::Yes, OrderKind::Ask, 1_000, 65);
        // now = 1000 > alice.expires 500 → alice dropped, no match.
        assert!(match_book(&[a, b], 100, 1_000).is_empty());
    }

    #[test]
    fn partial_fill_across_two_asks() {
        let orders = vec![
            bo("alice", OrderSide::Yes, OrderKind::Bid, 10_000, 70),
            bo("bob", OrderSide::Yes, OrderKind::Ask, 4_000, 60),
            bo("carol", OrderSide::Yes, OrderKind::Ask, 4_000, 65),
        ];
        let m = match_book(&orders, 100, 0);
        // bid 10k fills 4k @60 (bob) then 4k @65 (carol); 2k remains unfilled.
        assert_eq!(m.len(), 2);
        let total: u64 = m.iter().map(|x| match x { Match::Direct { amount, .. } => *amount, _ => 0 }).sum();
        assert_eq!(total, 8_000);
    }
}
