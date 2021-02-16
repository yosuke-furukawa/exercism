/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, PartialOrd)]
enum HandTypes {
    HighCard,
    OnePair,
    TwoPair,
    ThreeCard,
    Straight,
    Flush,
    FullHouse,
    FourCard,
    StraightFlush,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Clone)]
enum Suit {
    S,
    H,
    D,
    C,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Clone)]
struct Card {
    num: u8,
    suit: Suit,
}

#[derive(Debug, PartialEq, Clone)]
struct Hand<'a> {
    cards: HashSet<Card>,
    hand: &'a str,
}

impl Hand<'_> {
    fn check_straight_flush(&self, vector: Vec<Card>) -> (HandTypes, u8, Vec<Card>) {
        let is_flush = (0..5).all(|i| vector[0].suit == vector[i].suit);
        let is_straight = vector
            .windows(2)
            .all(|cards| cards[0].num + 1 == cards[1].num);
        let is_ace_straight = vector
            .windows(2)
            .all(|cards| cards[0].num + 1 == cards[1].num || cards[0].num + 9 == cards[1].num);
        if (is_straight || is_ace_straight) && is_flush {
            if !is_straight && is_ace_straight {
                return (
                    HandTypes::StraightFlush,
                    vector[vector.len() - 2].num,
                    vector,
                );
            }
            return (
                HandTypes::StraightFlush,
                vector[vector.len() - 1].num,
                vector,
            );
        } else if is_straight || is_ace_straight {
            if !is_straight && is_ace_straight {
                return (HandTypes::Straight, vector[vector.len() - 2].num, vector);
            }
            return (HandTypes::Straight, vector[vector.len() - 1].num, vector);
        } else if is_flush {
            return (HandTypes::Flush, vector[vector.len() - 1].num, vector);
        }
        (HandTypes::HighCard, vector[vector.len() - 1].num, vector)
    }
    fn check_fourcard_fullhouse(
        &self,
        map: HashMap<u8, i32>,
        vector: Vec<Card>,
    ) -> (HandTypes, u8, Vec<Card>) {
        for k in map.keys() {
            if map.get(k) == Some(&4) {
                return (HandTypes::FourCard, *k, vector);
            } else if map.get(k) == Some(&3) {
                return (HandTypes::FullHouse, *k, vector);
            } else {
                continue;
            }
        }
        panic!("no such hands");
    }
    fn check_threecard_twopair(
        &self,
        map: HashMap<u8, i32>,
        vector: Vec<Card>,
    ) -> (HandTypes, u8, Vec<Card>) {
        let mut two_pair_max = 0;
        for k in map.keys() {
            match map.get(k) {
                Some(&v) if v == 3 => return (HandTypes::ThreeCard, *k, vector),
                Some(&v) if v == 2 => {
                    two_pair_max = two_pair_max.max(*k);
                }
                _ => continue,
            }
        }
        (HandTypes::TwoPair, two_pair_max, vector)
    }
    fn check_onepair(
        &self,
        map: HashMap<u8, i32>,
        vector: Vec<Card>,
    ) -> (HandTypes, u8, Vec<Card>) {
        for k in map.keys() {
            match map.get(k) {
                Some(&v) if v == 2 => return (HandTypes::OnePair, *k, vector),
                _ => continue,
            }
        }
        panic!("no such hands");
    }
    fn check(&self) -> (HandTypes, u8, Vec<Card>) {
        let mut map = HashMap::new();
        let mut vector = vec![];

        for card in self.cards.clone() {
            *map.entry(card.num).or_insert(0) += 1;
            vector.push(card);
        }
        vector.sort_unstable_by(|a, b| a.num.cmp(&b.num));
        match map.len() {
            5 => Self::check_straight_flush(&self, vector),
            4 => Self::check_onepair(&self, map, vector),
            3 => Self::check_threecard_twopair(&self, map, vector),
            2 => Self::check_fourcard_fullhouse(&self, map, vector),
            _ => (HandTypes::HighCard, 0, vector),
        }
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (hand1, card1, vector1) = self.check();
        let (hand2, card2, vector2) = other.check();
        let result = hand1.partial_cmp(&hand2);
        if let Some(result) = result {
            if result == Ordering::Equal {
                let mut c = card1.partial_cmp(&card2);
                let mut i = 4;
                while c == Some(Ordering::Equal) {
                    c = Some(vector1[i].num.cmp(&vector2[i].num));
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                }
                return c;
            }
        }
        result
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut results = Vec::with_capacity(hands.len());
    for hand in hands {
        let h: Vec<&str> = hand.split(' ').collect();
        let mut cards: HashSet<Card> = HashSet::new();
        let c = h.iter().map(|card| {
            let chars: Vec<char> = card.chars().collect();
            let suit = match chars[chars.len() - 1] {
                'S' => Suit::S,
                'C' => Suit::C,
                'H' => Suit::H,
                'D' => Suit::D,
                _ => panic!("no such char"),
            };
            let num: u8 = match chars[0] {
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                n if chars.len() == 2 => n as u8 - b'0',
                _ => 10,
            };
            Card { num, suit }
        });
        for card in c {
            cards.insert(card);
        }
        results.push(Hand { cards, hand });
    }

    // win if hand is only one.
    if results.len() < 2 {
        return Some(results.iter().map(|h| h.hand).collect());
    }

    results.sort_unstable_by(|a, b| b.partial_cmp(&a).unwrap());
    let mut winners = HashSet::new();

    for candidates in results.windows(2) {
        let comp = candidates[0].partial_cmp(&candidates[1]);
        if comp == Some(Ordering::Equal) {
            winners.insert(candidates[0].hand);
            winners.insert(candidates[1].hand);
        } else {
            winners.insert(candidates[0].hand);
            break;
        }
    }
    Some(winners.into_iter().collect())
}
